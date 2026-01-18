# Rigs Project: Cost & Viability Analysis

> **Analysis Date**: January 2026
> **Budget Context**: Limited budget, considering ~1000 DKK (~$140 USD) per provider
> **Goal**: Determine if consumer subscriptions can power the Rigs multi-agent orchestration system

---

## Executive Summary

| Approach | Monthly Cost | Viability | Recommendation |
|----------|-------------|-----------|----------------|
| **Consumer Subs Only** | $140-240/mo | ⚠️ Risky | Rate limits may block heavy usage |
| **Hybrid (Subs + DeepSeek API)** | $140-180/mo | ✅ Good | Best balance of power and cost |
| **API-First + Local** | ~$50-100/mo | ✅ Excellent | Most flexible, requires more setup |
| **Wait for budget** | $0 now | ⏸️ Delay | 1-2 months to accumulate funds |

**TL;DR**: The plan is viable, but you should use DeepSeek API heavily for the Assayer layer (it's ~20-30x cheaper than Claude/OpenAI APIs) and reserve expensive subscriptions for complex tasks only.

---

## Provider-by-Provider Analysis

### 1. Claude (Anthropic)

#### Subscription Options

| Plan | Price | 5-Hour Window | Weekly Limits | Best For |
|------|-------|---------------|---------------|----------|
| **Pro** | $20/mo | ~45 messages, ~44K tokens | ~40-80 hrs Sonnet | Light usage, testing |
| **Max 5x** | $100/mo | ~225 messages, ~88K tokens | 140-280 hrs Sonnet, 15-35 hrs Opus | Moderate daily work |
| **Max 20x** | $200/mo | ~900 messages, ~220K tokens | 240-480 hrs Sonnet, 24-40 hrs Opus | Power users |

#### API Pricing (Pay-as-you-go)

| Model | Input (per 1M tokens) | Output (per 1M tokens) | Notes |
|-------|----------------------|------------------------|-------|
| Sonnet 4.5 | $3.00 | $15.00 | Best for coding |
| Opus 4.5 | $15.00 | $75.00 | Most capable, expensive |
| Haiku 4.5 | $0.25 | $1.25 | Fast, cheap |

#### Reality Check for Rigs
- **Pro ($20/mo)**: ~45 messages/5hr = ~216 messages/day max
  - This is **tight** for agentic coding workflows
  - Claude Code consumes tokens FAST (system prompts, file context, etc.)
  - You'd likely exhaust in 2-3 hours of active development
  
- **Max 5x ($100/mo)**: More realistic for daily use
  - 225 messages/5hr with weekly cap
  - ~15-35 hours of Opus/week is the real constraint
  - Viable for Rigs if you're strategic about when to use it

**Verdict**: Max 5x ($100/mo) is minimum for serious use. Pro is only for light work.

---

### 2. ChatGPT (OpenAI)

#### Subscription Options

| Plan | Price | Limits | Models | Notes |
|------|-------|--------|--------|-------|
| **Free** | $0 | 10 msgs/5hr GPT-5, then GPT-5-mini | GPT-5.2 limited | Very restrictive |
| **Plus** | $20/mo | Higher limits, priority | GPT-5.2 (Auto/Instant/Thinking) | Good for testing |
| **Pro** | $200/mo | "Unlimited" (fair use) | GPT-5.2 Pro, o1/o3 reasoning | Best reasoning |

#### API Pricing

| Model | Input (per 1M tokens) | Output (per 1M tokens) |
|-------|----------------------|------------------------|
| GPT-5.2 | ~$1.25 | ~$10.00 |
| GPT-4o | ~$2.50 | ~$10.00 |
| GPT-4o-mini | ~$0.15 | ~$0.60 |
| o1 (reasoning) | ~$15.00 | ~$60.00 |

#### Reality Check for Rigs
- **Plus ($20/mo)**: Good value, but limits are still tight for heavy coding
- **Pro ($200/mo)**: "Unlimited" but expensive
  - Best for: Research tasks, complex reasoning
  - Codex integration is powerful but burns through quota

**Verdict**: Plus ($20/mo) for supplementary use. Pro ($200) only if you need heavy reasoning.

---

### 3. Google Gemini

#### Subscription Options

| Plan | Price | Models | Notes |
|------|-------|--------|-------|
| **Free** | $0 | Gemini Flash 2.5 | Limited, but usable |
| **AI Pro** | $20/mo | Gemini 3 Pro, higher limits | Includes 2TB storage, Workspace integration |
| **AI Ultra** | $250/mo | Highest limits, Deep Think | Premium tier |

#### API Pricing (Very Competitive!)

| Model | Input (per 1M tokens) | Output (per 1M tokens) | Free Tier |
|-------|----------------------|------------------------|-----------|
| Gemini 3 Pro | $1.25 | $5.00 | 5 RPM, 25 RPD |
| Gemini 2.5 Flash | $0.075 | $0.30 | Very generous |
| Gemini 2.5 Pro | $1.25 | $5.00 | Limited |

#### Free API Tier (Developer)
- 5 requests/minute for Gemini 2.5 Pro
- 25 requests/day
- 15 RPM for Flash models
- **This is actually usable for light workloads!**

#### Reality Check for Rigs
- **AI Pro ($20/mo)**: Good value with Google Workspace integration
- **API Free Tier**: Can handle Assayer workloads if you're patient
- **Best for**: Research tasks, document analysis, search integration

**Verdict**: AI Pro ($20/mo) or even just the free API tier for Assayer/research tasks.

---

### 4. DeepSeek ⭐ (HIGHLY RECOMMENDED)

#### API Pricing (Extremely Cheap!)

| Model | Input (per 1M tokens) | Output (per 1M tokens) | Notes |
|-------|----------------------|------------------------|-------|
| V3.2 | $0.25 | $0.38 | Latest, very capable |
| V3.1 | $0.15 | $0.75 | Stable, good for coding |
| R1 (reasoning) | $0.45 | $2.15 | Chain-of-thought, math/code |
| R1 (cache hit) | $0.07 | $2.15 | Even cheaper with caching |

#### Free Tier
- 5 million free tokens on signup (~$8.40 value)
- Valid for 30 days
- No rate limits during trial

#### Why DeepSeek is Perfect for Rigs
1. **20-30x cheaper than Claude/OpenAI** for similar quality
2. **R1 model excels at reasoning** - perfect for Planner Assayer
3. **V3 is great for coding** - perfect for Optimizer Assayer
4. **Can run locally via Ollama** - completely FREE

#### Cost Comparison for Typical Rigs Session

| Task | Claude Sonnet | DeepSeek V3 | Savings |
|------|---------------|-------------|---------|
| 10K input, 2K output | $0.06 | $0.003 | 95% |
| 100K input, 20K output | $0.60 | $0.03 | 95% |
| Full Assayer pipeline | ~$2-5 | ~$0.10-0.25 | 95% |

**Verdict**: Use DeepSeek for ALL Assayer tasks. It's a no-brainer.

---

### 5. Local Models (Ollama) - FREE

#### Recommended Models

| Model | VRAM Needed | Quality | Best For |
|-------|-------------|---------|----------|
| DeepSeek-R1:7B | 4GB | ★★★★☆ | Planning, reasoning |
| Qwen3:8B | 5GB | ★★★★☆ | Code optimization |
| Llama3.2:3B | 2GB | ★★★☆☆ | Estimation, quick tasks |
| DeepSeek-V3 (distilled) | 8GB+ | ★★★★★ | Everything |

#### Setup
```bash
# Install Ollama (FREE)
curl -fsSL https://ollama.com/install.sh | sh

# Pull models (FREE)
ollama pull deepseek-r1:7b    # ~4GB
ollama pull qwen3:8b          # ~5GB
ollama pull llama3.2:3b       # ~2GB
```

**Verdict**: Run Assayer locally for $0/month. Quality is 80-90% of API for most tasks.

---

## Recommended Configurations

### Option A: Minimum Viable ($60-80/mo)
Best for: Budget-conscious, patient workflow

| Provider | Plan | Cost | Role |
|----------|------|------|------|
| Claude | Pro | $20/mo | Complex implementation tasks |
| ChatGPT | Plus | $20/mo | Research, reasoning backup |
| Gemini | Free API | $0/mo | Document analysis |
| DeepSeek | API | ~$10-20/mo | Assayer pipeline (heavy use) |
| Ollama | Local | $0 | Planner, Estimator, QualityGate |

**Total**: ~$50-60/mo + ~$10-20 API usage = **$60-80/mo**

**Limitations**: 
- Will hit Claude limits during intense coding sessions
- Need to be strategic about which provider handles which task

---

### Option B: Balanced Power ($140-180/mo) ⭐ RECOMMENDED
Best for: Your budget (~1000 DKK/mo)

| Provider | Plan | Cost | Role |
|----------|------|------|------|
| Claude | Max 5x | $100/mo | Primary coding, Claude Code |
| ChatGPT | Plus | $20/mo | Research, reasoning |
| Gemini | AI Pro | $20/mo | Deep Research, docs |
| DeepSeek | API | ~$5-15/mo | All Assayer tasks |
| Ollama | Local | $0 | Backup, quick tasks |

**Total**: ~$140 subscriptions + ~$5-15 API = **$145-155/mo** (~1050-1100 DKK)

**Benefits**:
- Max 5x gives you 225 messages/5hr for serious coding
- DeepSeek handles ALL optimization work cheaply
- Gemini handles research/documentation
- Never blocked - always have a fallback

---

### Option C: Maximum Power ($340-400/mo)
Best for: If budget allows later

| Provider | Plan | Cost | Role |
|----------|------|------|------|
| Claude | Max 20x | $200/mo | Primary everything |
| ChatGPT | Pro | $200/mo | Reasoning, o1/o3 |
| Gemini | AI Pro | $20/mo | Research |
| DeepSeek | API | ~$20-40/mo | Heavy Assayer usage |

**Total**: ~$420-460/mo

**Verdict**: Overkill for individual use. Only if money is no object.

---

## DKK Budget Analysis

Current exchange rate: 1 USD ≈ 7.0 DKK

| Budget | USD Equivalent | Best Configuration |
|--------|---------------|-------------------|
| 500 DKK/mo | ~$70 | Option A (Minimum) |
| **1000 DKK/mo** | ~$140 | **Option B (Balanced)** ⭐ |
| 1500 DKK/mo | ~$215 | Option B + ChatGPT Pro |
| 2000 DKK/mo | ~$285 | Option C (Maximum) |

---

## Rigs-Specific Token Estimates

Based on the Rigs architecture, here's estimated monthly token consumption:

### Assayer Layer (run on DeepSeek/Ollama)

| Component | Tokens/Task | Tasks/Day | Monthly Total | DeepSeek Cost |
|-----------|-------------|-----------|---------------|---------------|
| Planner | 3,000 | 5 | 450,000 | $0.17 |
| Optimizer | 2,000 | 20 | 1,200,000 | $0.45 |
| Estimator | 500 | 30 | 450,000 | $0.17 |
| QualityGate | 1,500 | 20 | 900,000 | $0.34 |
| **Total** | - | - | **3,000,000** | **$1.13/mo** |

**If run on Ollama locally**: $0/mo ✅

### Execution Layer (run on Pro subscriptions)

| Provider | Tasks/Day | Est. Tokens | Monthly | Fits in Plan? |
|----------|-----------|-------------|---------|---------------|
| Claude Max 5x | 3-5 | 50K-100K | 1.5-3M | ✅ Yes |
| ChatGPT Plus | 2-3 | 30K-50K | 0.9-1.5M | ✅ Yes |
| Gemini Pro | 2-3 | 30K-50K | 0.9-1.5M | ✅ Yes |

---

## Risk Analysis

### ⚠️ Potential Blockers

1. **Claude Weekly Caps**: Max 5x has a weekly cap of 15-35 hours Opus
   - **Mitigation**: Use Sonnet primarily (140-280 hrs/week), save Opus for complex tasks

2. **Rate Limit Variability**: All providers can throttle during peak
   - **Mitigation**: Multiple providers = always have a fallback

3. **API Cost Overruns**: Easy to spend more than expected
   - **Mitigation**: DeepSeek has hard budget caps, set alerts

4. **Local Model Quality**: Ollama models aren't as good as API
   - **Mitigation**: Use local for simple tasks, API for complex

### ✅ Why It Will Work

1. **DeepSeek changes the economics**: 95% cheaper = Assayer costs ~$1-5/mo
2. **Subscription pooling**: Using 3 providers means rarely blocked
3. **Smart routing**: Rigs dispatches to whoever has capacity
4. **Local fallback**: Ollama means you're never truly blocked

---

## Recommended Phased Approach

### Phase 1: Start Free (Month 1)
- Use Ollama for all Assayer work
- Use free tiers of Gemini, DeepSeek, ChatGPT
- Use Claude Pro ($20) for implementation
- **Cost**: $20/mo

### Phase 2: Add DeepSeek API (Month 2)
- Add DeepSeek API for better Assayer quality
- Keep Claude Pro
- **Cost**: $25-35/mo

### Phase 3: Upgrade to Full Power (Month 3+)
- Upgrade Claude to Max 5x
- Add ChatGPT Plus or Gemini Pro
- **Cost**: ~$140-155/mo (Option B)

---

## Conclusion

### Is the Rigs project viable?

**YES**, with caveats:

1. **Use DeepSeek API heavily** - It's 20-30x cheaper than competitors
2. **Run Assayer on Ollama locally** - Completely free
3. **Claude Max 5x is the sweet spot** - $100/mo for serious coding
4. **Multi-provider approach is essential** - Never be blocked

### Recommended Starting Configuration

```
Month 1-2 (Testing):
├── Claude Pro: $20/mo
├── Ollama: FREE (Assayer)
├── DeepSeek API: ~$5/mo
└── Gemini Free: $0
Total: ~$25/mo

Month 3+ (Production):
├── Claude Max 5x: $100/mo
├── ChatGPT Plus: $20/mo
├── Gemini AI Pro: $20/mo
├── DeepSeek API: ~$10/mo
└── Ollama: FREE (backup)
Total: ~$150/mo (~1050 DKK)
```

### Final Recommendation

**Start with Option A ($60-80/mo) for 1-2 months while building Rigs, then upgrade to Option B ($140-155/mo) when ready for production use.**

The key insight is that DeepSeek and Ollama make the Assayer layer essentially free, so your subscription budget can go entirely toward execution quality.

---

## DeepSeek V4 / Future Models

You mentioned waiting for DeepSeek V4. Based on their release cadence:

- V3 released Dec 2024
- V3.1 released Aug 2025
- V3.2 released Dec 2025
- **V4 likely: Q2-Q3 2026**

If V4 follows the pattern, expect:
- 2-3x better quality
- Similar or lower pricing
- Potential free tier expansion

**Recommendation**: Don't wait. Start now with V3.2, which is already excellent. Upgrade when V4 drops.
