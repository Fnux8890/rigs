//! Tank (rate limit state) types
//!
//! A Tank tracks the current rate limit state for a single provider.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use super::provider::Provider;

/// Health level of a tank based on remaining capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TankHealth {
    /// >50% capacity remaining
    Green,
    /// 20-50% capacity remaining
    Yellow,
    /// <20% capacity remaining
    Red,
    /// 0% capacity - locked out
    Empty,
}

impl TankHealth {
    /// Calculate health from capacity ratio and thresholds
    pub fn from_ratio(ratio: f32, yellow_threshold: f32, red_threshold: f32) -> Self {
        if ratio <= 0.0 {
            TankHealth::Empty
        } else if ratio < red_threshold {
            TankHealth::Red
        } else if ratio < yellow_threshold {
            TankHealth::Yellow
        } else {
            TankHealth::Green
        }
    }

    /// Get ANSI color code for terminal display
    pub fn color_code(&self) -> &'static str {
        match self {
            TankHealth::Green => "\x1b[32m",  // Green
            TankHealth::Yellow => "\x1b[33m", // Yellow
            TankHealth::Red => "\x1b[31m",    // Red
            TankHealth::Empty => "\x1b[90m",  // Gray
        }
    }

    /// Get emoji for display
    pub fn emoji(&self) -> &'static str {
        match self {
            TankHealth::Green => "🟢",
            TankHealth::Yellow => "🟡",
            TankHealth::Red => "🔴",
            TankHealth::Empty => "⚫",
        }
    }
}

/// Rate limit state for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tank {
    /// Provider this tank belongs to
    pub provider: Provider,
    /// Maximum tokens in window
    pub capacity: u64,
    /// Remaining tokens in current window
    pub remaining: u64,
    /// When current window started
    pub window_start: DateTime<Utc>,
    /// When current window ends (reset time)
    pub window_end: DateTime<Utc>,
    /// Current health level
    pub health: TankHealth,
    /// Last API request time
    pub last_request: Option<DateTime<Utc>>,
    /// Requests made in current window
    pub requests_this_window: u32,
    /// Tokens consumed in current window
    pub tokens_this_window: u64,
    /// When this tank state was last updated
    pub updated_at: DateTime<Utc>,
}

impl Tank {
    /// Create a new tank with full capacity
    pub fn new(provider: Provider, capacity: u64, window_hours: u32) -> Self {
        let now = Utc::now();
        let window_end = now + Duration::hours(window_hours as i64);

        Self {
            provider,
            capacity,
            remaining: capacity,
            window_start: now,
            window_end,
            health: TankHealth::Green,
            last_request: None,
            requests_this_window: 0,
            tokens_this_window: 0,
            updated_at: now,
        }
    }

    /// Get current capacity ratio (0.0 to 1.0)
    pub fn capacity_ratio(&self) -> f32 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.remaining as f32 / self.capacity as f32
    }

    /// Get time until window resets
    pub fn time_until_reset(&self) -> Duration {
        let now = Utc::now();
        if now >= self.window_end {
            Duration::zero()
        } else {
            self.window_end - now
        }
    }

    /// Check if window has reset and needs refresh
    pub fn needs_refresh(&self) -> bool {
        Utc::now() >= self.window_end
    }

    /// Check if we can consume the given number of tokens
    pub fn can_consume(&self, tokens: u64) -> bool {
        self.remaining >= tokens && self.health != TankHealth::Empty
    }

    /// Consume tokens from the tank
    pub fn consume(&mut self, tokens: u64) -> Result<(), InsufficientCapacity> {
        if !self.can_consume(tokens) {
            return Err(InsufficientCapacity {
                requested: tokens,
                available: self.remaining,
                provider: self.provider,
            });
        }

        self.remaining = self.remaining.saturating_sub(tokens);
        self.tokens_this_window += tokens;
        self.requests_this_window += 1;
        self.last_request = Some(Utc::now());
        self.recalculate_health(0.5, 0.2); // Default thresholds
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Reset the window (call when window_end is reached)
    pub fn reset_window(&mut self, window_hours: u32) {
        let now = Utc::now();
        self.window_start = now;
        self.window_end = now + Duration::hours(window_hours as i64);
        self.remaining = self.capacity;
        self.requests_this_window = 0;
        self.tokens_this_window = 0;
        self.recalculate_health(0.5, 0.2);
        self.updated_at = now;
    }

    /// Update remaining capacity (e.g., from API response)
    pub fn update_remaining(&mut self, remaining: u64, yellow: f32, red: f32) {
        self.remaining = remaining.min(self.capacity);
        self.recalculate_health(yellow, red);
        self.updated_at = Utc::now();
    }

    /// Recalculate health based on current ratio
    fn recalculate_health(&mut self, yellow_threshold: f32, red_threshold: f32) {
        self.health = TankHealth::from_ratio(self.capacity_ratio(), yellow_threshold, red_threshold);
    }

    /// Generate a progress bar string (for display)
    pub fn progress_bar(&self, width: usize) -> String {
        let ratio = self.capacity_ratio();
        let filled = (ratio * width as f32).round() as usize;
        let empty = width - filled;

        let fill_char = match self.health {
            TankHealth::Green => '█',
            TankHealth::Yellow => '▓',
            TankHealth::Red => '▒',
            TankHealth::Empty => '░',
        };

        format!(
            "[{}{}] {:3.0}%",
            fill_char.to_string().repeat(filled),
            '░'.to_string().repeat(empty),
            ratio * 100.0
        )
    }
}

/// Error when trying to consume more tokens than available
#[derive(Debug, Clone)]
pub struct InsufficientCapacity {
    pub requested: u64,
    pub available: u64,
    pub provider: Provider,
}

impl std::fmt::Display for InsufficientCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Insufficient capacity on {}: requested {} tokens, only {} available",
            self.provider, self.requested, self.available
        )
    }
}

impl std::error::Error for InsufficientCapacity {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tank_creation() {
        let tank = Tank::new(Provider::Claude, 100_000, 5);
        assert_eq!(tank.capacity, 100_000);
        assert_eq!(tank.remaining, 100_000);
        assert_eq!(tank.health, TankHealth::Green);
    }

    #[test]
    fn test_tank_consumption() {
        let mut tank = Tank::new(Provider::Claude, 100_000, 5);
        
        assert!(tank.consume(50_000).is_ok());
        assert_eq!(tank.remaining, 50_000);
        assert_eq!(tank.health, TankHealth::Yellow);
        
        assert!(tank.consume(40_000).is_ok());
        assert_eq!(tank.remaining, 10_000);
        assert_eq!(tank.health, TankHealth::Red);
        
        // Can't consume more than available
        assert!(tank.consume(20_000).is_err());
    }

    #[test]
    fn test_tank_health() {
        assert_eq!(TankHealth::from_ratio(0.6, 0.5, 0.2), TankHealth::Green);
        assert_eq!(TankHealth::from_ratio(0.3, 0.5, 0.2), TankHealth::Yellow);
        assert_eq!(TankHealth::from_ratio(0.1, 0.5, 0.2), TankHealth::Red);
        assert_eq!(TankHealth::from_ratio(0.0, 0.5, 0.2), TankHealth::Empty);
    }

    #[test]
    fn test_progress_bar() {
        let mut tank = Tank::new(Provider::Claude, 100, 5);
        tank.remaining = 75;
        tank.recalculate_health(0.5, 0.2);
        
        let bar = tank.progress_bar(10);
        assert!(bar.contains("75%"));
    }
}
