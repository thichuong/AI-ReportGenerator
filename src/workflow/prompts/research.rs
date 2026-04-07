//! Research prompts

pub const COMBINED_RESEARCH_PROMPT: &str = r#"# 📊 Prompt Kết hợp: Research + Validation - Thị Trường Crypto

## 🎯 Thông Tin Cơ Bản

**Vai trò:** Nhà phân tích Thị trường Tài chính Cấp cao với Khả năng Tự kiểm tra Real-time
**Nhiệm vụ:** 
1. Soạn thảo Báo cáo Nghiên cứu Chuyên sâu về thị trường crypto cho ngày hôm nay
2. Tự động xác thực và điều chỉnh báo cáo dựa trên dữ liệu thời gian thực
3. Đảm bảo báo cáo cuối cùng có độ chính xác cao
4. **Tìm thêm thông tin từ các nguồn uy tín toàn cầu** - Đa dạng hóa nguồn từ Bloomberg, Reuters, Financial Times, Wall Street Journal, CoinDesk, The Block, Decrypt, CryptoSlate
5. **Tăng cường phân tích tin tức chính trị, kinh tế, crypto** - Theo dõi sát sao các sự kiện địa chính trị, chính sách tiền tệ, quy định pháp lý, và tin tức breaking news có thể gây biến động thị trường
6. **Giám sát hoạt động giao dịch của các tổ chức lớn** - Phân tích chi tiết hoạt động mua bán của quỹ đầu tư, công ty niêm yết, cá voi (whale), và các tổ chức tài chính lớn

**Ngày báo cáo:** Hôm nay ngày <<@day>> tháng <<@month>> năm <<@year>>

---

## 📊 DỮ LIỆU THỜI GIAN THỰC THAM KHẢO

**Dữ liệu cập nhật từ hệ thống dashboard:**

```json
{{REAL_TIME_DATA}}
```

**Hướng dẫn sử dụng:** 
- Sử dụng dữ liệu này để cross-reference với thông tin thu thập được từ Google Search
- Ưu tiên dữ liệu real-time cho các số liệu cơ bản (giá BTC, Fear & Greed Index, Market Cap)
- Nếu có sự khác biệt, hãy note và explain trong báo cáo

---

## 🌐 CHIẾN LƯỢC THU THẬP THÔNG TIN ĐA NGUỒN

### 📰 Nguồn Tin Uy Tín Bắt Buộc (Tier 1 Sources)

**🏛️ Tài chính Truyền thống:**
- **Bloomberg Terminal/Bloomberg Crypto** - Tin tức thể chế, phân tích chuyên sâu
- **Reuters Markets** - Breaking news toàn cầu, chính sách tiền tệ
- **Financial Times** - Phân tích vĩ mô, xu hướng institutional adoption
- **Wall Street Journal** - Regulatory updates, corporate crypto strategies
- **MarketWatch** - Real-time market sentiment, analyst opinions

**💎 Crypto Native Sources:**
- **CoinDesk** - Industry news, regulatory developments, institutional moves
- **The Block** - On-chain analysis, venture funding, institutional research
- **Decrypt** - Technology developments, DeFi innovations
- **CryptoSlate** - Project updates, funding rounds, market analysis
- **Coinbase Research** - Professional-grade institutional insights

**📊 Data & Analytics Platforms:**
- **Glassnode** - On-chain metrics, whale activity, network health
- **CryptoQuant** - Exchange flows, miner data, institutional indicators
- **Santiment** - Social sentiment, developer activity, network growth
- **Messari** - Fundamental analysis, tokenomics, protocol revenue
- **Nansen** - Wallet tracking, smart money flows, DeFi analytics

### 🔍 Tin Tức Ảnh Hưởng Thị Trường - Monitoring Keywords

**🏛️ Chính Trị & Chính Sách:**
- **Search Terms**: "Federal Reserve crypto", "SEC Bitcoin ETF", "Congress crypto hearing", "Biden crypto executive order", "Trump crypto policy", "election crypto impact"
- **Global Politics**: "China crypto ban", "EU MiCA regulation", "Russia crypto sanctions", "Ukraine crypto aid", "Middle East crypto adoption"
- **Regulatory Milestones**: "stablecoin regulation", "DeFi regulation", "crypto tax policy", "CBDC development", "banking crypto guidelines"

**💰 Kinh Tế Vĩ Mô:**
- **Search Terms**: "inflation crypto correlation", "Federal Reserve meeting crypto", "interest rates Bitcoin", "dollar strength crypto", "recession Bitcoin hedge"
- **Economic Indicators**: "unemployment crypto market", "GDP growth Bitcoin", "PMI crypto correlation", "oil prices crypto", "gold vs Bitcoin"
- **Banking Crisis**: "bank failures crypto surge", "credit crunch Bitcoin", "liquidity crisis crypto safe haven"

**⚡ Breaking Crypto News:**
- **Search Terms**: "Bitcoin ETF approval", "crypto exchange hack", "major Bitcoin purchase", "crypto company bankruptcy", "stablecoin depeg"
- **Technology Updates**: "Bitcoin upgrade", "Ethereum merge update", "layer 2 adoption", "DeFi exploit", "NFT market trends"
- **Celebrity/Influencer Impact**: "Elon Musk crypto tweet", "Michael Saylor Bitcoin", "institutional crypto adoption", "CEO crypto comments"

### 🐋 Giám Sát Hoạt Động Cá Voi & Tổ Chức

**💼 Corporate Treasury Movements:**
- **Search Terms**: "MicroStrategy Bitcoin purchase", "Tesla Bitcoin holdings", "Square crypto investment", "corporate Bitcoin treasury", "public company crypto adoption"
- **Tracking Requirements**:
  - Quarterly earnings calls mentioning crypto
  - SEC filing searches (8-K, 10-Q, 10-K forms)
  - Corporate press releases về crypto investments
  - Board resolution announcements
  - CFO statements về crypto strategy

**🏦 Institutional Trading Intelligence:**
- **Search Terms**: "Goldman Sachs crypto trading", "JPMorgan Bitcoin", "Morgan Stanley crypto", "BlackRock Bitcoin ETF", "Fidelity crypto institutional"
- **Prime Brokerage Activity**:
  - Large OTC transaction reports
  - Institutional custody solution announcements
  - Prime services crypto expansion news
  - Qualified custodian registrations
  - Institutional trading platform launches

**🐋 Whale Activity Monitoring:**
- **On-chain Tracking Keywords**: "Bitcoin whale movements", "large Bitcoin transactions", "dormant Bitcoin wallets", "exchange whale deposits", "whale accumulation patterns"
- **Analysis Requirements**:
  - Transactions >100 BTC identification
  - Whale wallet behavior changes
  - Exchange vs cold storage movements
  - Long-term holder distribution changes
  - Whale cluster analysis và smart money flows

**💰 Investment Fund Activities:**
- **Search Terms**: "crypto hedge fund", "Bitcoin fund performance", "institutional crypto fund", "pension fund crypto", "endowment crypto allocation"
- **Fund Categories**:
  - **Hedge Funds**: Pantera, Galaxy Digital, Bitwise, Grayscale funds
  - **Asset Managers**: BlackRock, Fidelity, VanEck crypto products
  - **Pension Funds**: Public pension crypto allocations
  - **Endowments**: University endowment crypto investments
  - **Family Offices**: Ultra-high-net-worth crypto adoption

**📊 Market Maker & Trading Firm Intelligence:**
- **Search Terms**: "Jump Trading crypto", "Alameda Research", "Cumberland crypto", "market maker crypto volumes", "crypto trading firm news"
- **Activities to Track**:
  - Market making announcements
  - Trading volume disclosures
  - New market partnerships
  - Technology infrastructure investments
  - Regulatory compliance updates

---

## 📋 QUY TRÌNH THỰC HIỆN (2 GIAI ĐOẠN)

### GIAI ĐOẠN 1: 🔍 RESEARCH & DATA COLLECTION

Thực hiện research chi tiết với các phần sau:

#### 1.1 📈 Tóm tắt báo cáo (Executive Summary)
- Cung cấp cái nhìn **tổng quan ngắn gọn** về những diễn biến quan trọng nhất của thị trường trong **24 giờ qua**
- Nêu bật **kết luận chính** và **triển vọng ngắn hạn**
- **Cross-check với real-time data**: Sử dụng dữ liệu BTC price và 24h change từ hệ thống để verify

#### 1.2 🧠 Phân tích Tâm lý Thị trường

**Chỉ số Sợ hãi & Tham lam (Fear & Greed Index):**
- Lấy chỉ số mới nhất từ Google Search
- **Cross-check**: So sánh với dữ liệu FNG từ hệ thống real-time
- Phân tích ý nghĩa của mức chỉ số hiện tại so với lịch sử (ngày hôm qua, tuần trước)
- Tác động của nó đến hành vi của nhà đầu tư
- Diễn giải mức độ sợ hãi/tham lam và correlation với price action

#### 1.3 📊 Phân tích Kỹ thuật Chuyên sâu

**🟡 Bitcoin Dominance (BTC.D):**
- **Cross-check**: So sánh dữ liệu BTC.D từ Google Search với real-time data
- Phân tích xu hướng BTC.D và ý nghĩa với altcoin season
- Tác động của việc tăng/giảm dominance đến toàn bộ thị trường
- So sánh với mức lịch sử và các mốc quan trọng (45%, 50%, 55%)

**🕯️ Phân tích Nến Weekly:**
- **Cross-check**: Xác thực giá đóng cửa tuần và patterns từ real-time data
- Phân tích pattern nến weekly của BTC và top altcoins
- Xác định support/resistance levels quan trọng
- Đánh giá momentum và trend strength trên timeframe weekly

**📈 Volume Analysis:**
- **Cross-check**: So sánh volume data với real-time metrics
- Phân tích volume profile và unusual volume spikes
- Volume-price correlation và confirmation signals
- On-chain volume vs exchange volume analysis

**⚡ RSI và Momentum Indicators:**
- **Cross-check**: Xác thực các chỉ báo kỹ thuật với real-time data
- RSI levels trên multiple timeframes (daily, weekly)
- Divergence analysis và overbought/oversold conditions
- MACD, Stochastic và các momentum indicators khác

#### 1.4 💰 Dòng tiền Tổ chức & Phân tích Cá Voi (Enhanced Institutional & Whale Analysis)

**📊 Bitcoin ETF Data - Chi tiết theo từng Provider:**
- **Cross-check**: So sánh ETF flows từ Google Search với real-time institutional data
- **Search keywords**: "Bitcoin ETF flows today", "GBTC FBTC ARKB BITO flows", "Bitcoin ETF inflows outflows"
- Phân tích net inflows/outflows của TỪNG Bitcoin ETF cụ thể:
  - **Grayscale GBTC**: Volume, fee structure impact, outflow patterns
  - **Fidelity FBTC**: Competitive positioning, growth trajectory
  - **ARK ARKB**: Innovation focus, retail vs institutional flows
  - **ProShares BITO**: Futures-based ETF performance comparison
  - **iShares IBIT**: BlackRock institutional client adoption
- So sánh AUM changes, expense ratios, và tracking error
- Phân tích correlation giữa ETF flows và BTC spot price movements
- Institutional demand impact on price discovery và market microstructure

**🐋 Whale Movement Analysis - Comprehensive Tracking:**
- **Cross-check**: Validate whale activity từ multiple on-chain sources với real-time blockchain data
- **Search keywords**: "Bitcoin whale movements today", "large Bitcoin transactions", "whale accumulation Bitcoin", "dormant Bitcoin wallets activated"
- **Large Transaction Monitoring (>100 BTC)**:
  - Transaction value, timestamp, và origin/destination analysis
  - Exchange deposits vs withdrawals patterns từ whale wallets
  - Cold storage accumulation vs distribution trends
  - Whale cluster behavior và coordinated movements
  - Long-term holder (LTH) vs short-term holder (STH) dynamics
- **Whale Wallet Behavior Analysis**:
  - Top 100 Bitcoin addresses activity tracking
  - Dormant wallet reactivation (>2 years inactive)
  - Whale profit-taking patterns at resistance levels
  - Accumulation phases during market dips
  - Cross-chain whale activity (BTC to ETH, stablecoins)
- **Smart Money Flow Indicators**:
  - Nansen "Smart Money" wallet tracking
  - Fund wallet activity correlation với price movements
  - Whale vs retail sentiment divergence analysis
  - Early adopter wallet behavior patterns

**🏦 Corporate Treasury Deep Dive:**
- **Cross-check**: Xác thực corporate holdings với real-time SEC filings và earnings data
- **Search keywords**: "MicroStrategy Bitcoin purchase", "corporate Bitcoin treasury 2024", "public company crypto holdings", "earnings call crypto mentions"
- **Public Company Bitcoin Holdings Tracking**:
  - **MicroStrategy (MSTR)**: Latest purchases, DCA strategy, financing methods
  - **Tesla (TSLA)**: Holdings changes, Elon Musk statements impact
  - **Block (SQ)**: Square crypto initiatives, Cash App Bitcoin metrics
  - **Coinbase (COIN)**: Corporate holdings vs customer holdings
  - **Marathon Digital, Riot Platforms**: Mining company treasury strategies
- **Corporate Adoption Pipeline Analysis**:
  - New corporate announcements quarterly tracking
  - Board resolution filings về crypto investment approval
  - CFO/CEO statements về future crypto strategy
  - Shareholder meeting discussions về crypto allocation
  - Credit rating impact từ crypto treasury decisions

**🏛️ Institutional Trading Patterns - Advanced Analytics:**
- **Cross-check**: So sánh institutional flow data với real-time prime brokerage metrics
- **Search keywords**: "institutional Bitcoin trading volume", "crypto prime brokerage", "Wall Street crypto adoption", "bank crypto trading desk"
- **Prime Brokerage Activity Indicators**:
  - Goldman Sachs, JPMorgan, Morgan Stanley crypto desk activity
  - Institutional OTC trading volume estimates
  - Custody solution adoption rates (Coinbase Prime, BitGo, Anchorage)
  - Prime services revenue growth từ crypto activities
  - Compliance infrastructure development investments
- **Institutional Flow Analysis**:
  - Large block trades (>$10M) identification và timing
  - Cross-venue arbitrage activity từ institutional traders
  - Institutional vs retail flow separation on major exchanges
  - After-hours trading patterns indicating institutional activity
  - Options flow analysis indicating institutional hedging strategies

**💼 Investment Vehicle Ecosystem:**
- **Cross-check**: Validate fund performance data với real-time AUM và flow metrics
- **Search keywords**: "crypto hedge fund performance", "Bitcoin fund flows", "institutional crypto fund AUM", "pension fund crypto allocation"
- **Hedge Fund Performance Tracking**:
  - Pantera Capital, Galaxy Digital, Polychain portfolio updates
  - Fund launches và closures impact on market sentiment
  - Performance attribution: alpha generation vs beta exposure
  - Fund redemption patterns during market stress
  - New fund registration và regulatory approvals
- **Pension & Endowment Adoption**:
  - Public pension fund crypto allocation announcements
  - University endowment crypto investment decisions
  - Sovereign wealth fund crypto exposure expansion
  - Insurance company crypto investment policy changes
  - Municipal government Bitcoin treasury adoptions

**️ Infrastructure Investment Intelligence:**
- **Search keywords**: "crypto infrastructure funding", "blockchain infrastructure investment", "crypto venture capital", "mining infrastructure expansion"
- **Infrastructure Development Tracking**:
  - Mining operation expansion announcements và financing
  - Data center partnerships for crypto mining
  - Energy partnership developments affecting mining sector
  - Custody infrastructure investment và technology upgrades
  - Exchange infrastructure scaling investments

#### 1.5 🌍 Phân tích Vĩ mô Chuyên sâu (Comprehensive Macro Analysis)

**🏛️ Chính sách Fed và Monetary Policy Impact:**
- **Cross-check**: So sánh Fed policy updates từ Google với real-time macro indicators
- **Search keywords**: "Federal Reserve latest decision", "FOMC meeting crypto impact", "interest rates Bitcoin correlation"
- **Fed Communications Analysis**:
  - Latest FOMC meeting minutes và dot plot analysis
  - Fed Chair Powell speeches và congressional testimony
  - Regional Fed president statements về digital assets
  - Federal Reserve research papers on cryptocurrency
- **Interest Rate Environment**:
  - Real yields vs nominal yields impact on crypto
  - Term structure changes và curve steepening/flattening
  - Fed funds futures market expectations
  - Credit spread analysis và liquidity conditions
- **Dollar Strength Analysis**:
  - DXY correlation với crypto performance (rolling 30/60/90 day)
  - Dollar milkshake theory implications
  - International capital flow patterns
  - Currency war dynamics affecting risk assets

**📜 Regulatory Landscape Deep Dive:**
- **Cross-check**: Validate regulatory developments với real-time compliance tracking
- **Search keywords**: "SEC crypto regulation latest", "Bitcoin ETF approval news", "crypto legal developments"
- **US Regulatory Developments**:
  - SEC enforcement actions và settlement patterns
  - CFTC jurisdiction clarifications
  - Congressional hearing outcomes và proposed legislation
  - State-level regulatory developments (Wyoming, Miami, etc.)
  - Presidential administration policy positions
- **Global Regulatory Coordination**:
  - EU MiCA implementation timeline và compliance requirements
  - UK crypto regulation framework development
  - Asian regulatory approaches (Japan, Singapore, Hong Kong)
  - Basel Committee banking regulations on crypto exposure
  - IMF và World Bank positions on digital assets
- **Regulatory Impact Assessment**:
  - Market reaction patterns to regulatory announcements
  - Compliance cost implications for industry players
  - Innovation impact từ regulatory clarity/uncertainty

**💰 Macro Economic Indicators Comprehensive View:**
- **Cross-check**: Verify economic data từ Google với real-time economic databases
- **Search keywords**: "inflation data crypto correlation", "employment report Bitcoin", "GDP growth cryptocurrency"
- **Inflation Dynamics**:
  - CPI, Core CPI, PCE deflator trends và crypto correlation
  - Producer Price Index và input cost pressures
  - Inflation expectations (5Y5Y breakeven, TIPS spreads)
  - Global inflation synchronization patterns
- **Employment & Growth Indicators**:
  - Non-farm payrolls và unemployment rate trends
  - Labor force participation và wage growth
  - GDP growth trajectories (US, EU, China, Japan)
  - Manufacturing PMI và services PMI global readings
- **Geopolitical Risk Factors**:
  - US-China trade relationship developments
  - Russia-Ukraine conflict economic implications
  - Middle East tensions và energy price impacts
  - Taiwan semiconductor supply chain considerations
  - Election cycles trong major economies

**🌐 Global Liquidity & Capital Flows:**
- **Search keywords**: "global liquidity conditions", "central bank balance sheets", "capital flows emerging markets"
- **Central Bank Policy Coordination**:
  - ECB policy divergence từ Federal Reserve
  - Bank of Japan yield curve control modifications
  - People's Bank of China monetary policy shifts
  - Emerging market central bank responses
- **Cross-border Capital Flow Analysis**:
  - Portfolio flows into/out of US assets
  - Foreign exchange intervention patterns
  - Cryptocurrency as international settlement mechanism
  - Stablecoin adoption trong international trade

**📅 Economic Calendar & Event Risk Management:**
- **Search keywords**: "economic calendar this week", "earnings season crypto impact"
- **Upcoming High-Impact Events**:
  - Central bank meeting schedules và policy announcement dates
  - Major economic data release calendar
  - Earnings seasons và crypto-exposed company reports
  - Options expiration dates với significant crypto positioning
- **Seasonal Pattern Analysis**:
  - Historical performance patterns by month/quarter
  - Tax season implications (April, year-end)
  - Holiday trading patterns và liquidity considerations
  - "Uptober" và other crypto-specific seasonal trends

**🔍 Advanced Correlation & Risk Analysis:**
- **Search keywords**: "Bitcoin stock market correlation", "crypto traditional asset correlation", "risk-on risk-off crypto"
- **Multi-asset Correlation Tracking**:
  - BTC correlation với S&P 500, Nasdaq, Gold, Bonds
  - Rolling correlation analysis (30, 60, 90, 180 day windows)
  - Correlation breakdown scenarios và implications
  - Cross-asset volatility spillover effects
- **Risk Sentiment Indicators**:
  - VIX và crypto volatility relationships
  - High yield credit spreads như risk appetite proxy
  - Commodity complex performance signals
  - Currency carry trade dynamics

#### 1.6  Phân tích Top Coins Chi tiết

**₿ Bitcoin (BTC) - Market Leader:**
- **Cross-check**: Verify BTC metrics với real-time price, volume, on-chain data
- Price action analysis và key support/resistance levels
- On-chain metrics: Network hash rate, difficulty adjustments, miner activity
- Institutional flows và ETF impact on BTC price
- Correlation với traditional assets và macro factors

**📱 Ethereum (ETH) - Smart Contract King:**
- **Cross-check**: So sánh ETH data với real-time DeFi metrics
- Ethereum ecosystem health: Gas fees, network utilization, DeFi TVL
- ETH 2.0 staking metrics và supply dynamics
- Layer 2 adoption impact on main chain activity
- DeFi protocol performance và innovation trends

**🔗 Major Altcoins Analysis:**
- **Cross-check**: Verify top 10-20 altcoin performance với real-time data
- Sector rotation analysis (DeFi, Layer 1s, Memes, AI tokens)
- Relative strength analysis vs BTC và ETH
- Fundamental developments và ecosystem growth
- Technical levels và breakout/breakdown patterns

**🎯 Emerging Trends:**
- **Cross-check**: Validate narrative shifts với real-time social sentiment
- Sector-specific analysis (AI, RWA, Gaming, Social)
- New protocol launches và innovation assessment
- Market cap distribution changes
- Narrative và theme-based performance analysis

#### 1.7 🗞️ Tin Tức Breaking & Phân tích Tác động (Breaking News Impact Analysis)

**⚡ Breaking News Monitoring - Real-time:**
- **Cross-check**: Validate breaking news từ multiple Tier 1 sources
- **Search keywords**: "crypto breaking news today", "Bitcoin news alert", "cryptocurrency market news", "SEC crypto announcement", "Federal Reserve crypto"
- **Critical News Categories**:
  - **Regulatory Bombshells**: SEC enforcement, new regulations, court decisions
  - **Institutional Announcements**: Major fund launches, corporate adoptions, bank partnerships
  - **Technical Developments**: Network upgrades, security incidents, protocol changes
  - **Geopolitical Events**: Government actions, international agreements, sanctions
  - **Market Structure Changes**: Exchange listings, trading halts, liquidity events

**📺 Multi-Source News Validation Process:**
- **Primary Verification** (Within 30 minutes):
  - Bloomberg Terminal alerts và professional news feeds
  - Reuters breaking news notifications
  - CoinDesk real-time news feed
  - Official announcements từ regulatory bodies
- **Secondary Confirmation** (Within 1 hour):
  - Cross-reference với 3+ independent tier 1 sources
  - Official statements từ involved parties
  - Social media verification từ verified accounts
  - Market reaction analysis để confirm news impact

**🎯 News Impact Scoring System:**
- **High Impact (Score 8-10)**: Fed decisions, major regulatory changes, large institutional adoptions
- **Medium Impact (Score 5-7)**: Corporate earnings, exchange announcements, technical upgrades
- **Low Impact (Score 1-4)**: Minor regulatory updates, small partnerships, routine operations
- **Market Reaction Timeline**: Immediate (0-15 min), Short-term (1-24h), Medium-term (1-7 days)

#### 1.8 🎙️ Ý kiến Chuyên gia & Sentiment Đa chiều (Multi-dimensional Expert Analysis)

**📺 Institutional Analyst Coverage - Wall Street Perspective:**
- **Cross-check**: So sánh institutional research với real-time price action và sentiment data
- **Search keywords**: "JPMorgan Bitcoin research", "Goldman Sachs crypto outlook", "Morgan Stanley Bitcoin price target", "Bank of America crypto report"
- **Major Bank Research Coverage**:
  - **JPMorgan**: Nikolaos Panigirtzoglou Bitcoin fair value models
  - **Goldman Sachs**: Crypto trading desk insights, institutional client demand
  - **Morgan Stanley**: Wealth management crypto recommendations
  - **Bank of America**: Digital asset research, regulatory impact analysis
  - **Citi**: Global crypto market outlook, central bank digital currency research
- **Rating Agency Perspectives**:
  - Standard & Poor's crypto credit ratings impact
  - Moody's blockchain adoption assessments
  - Fitch ratings on crypto exposure of traditional institutions

**🎯 Crypto Native Expert Analysis:**
- **Cross-check**: Validate expert predictions với real-time on-chain data và market performance
- **Search keywords**: "Michael Saylor Bitcoin prediction", "Cathie Wood crypto outlook", "Anthony Pompliano Bitcoin", "Raoul Pal crypto macro"
- **Tier 1 Crypto Thought Leaders**:
  - **Michael Saylor (MicroStrategy)**: Corporate treasury strategy, Bitcoin monetization
  - **Cathie Wood (ARK Invest)**: Innovation adoption curves, disruptive technology analysis
  - **Anthony Pompliano**: Institutional adoption timeline, macroeconomic correlations
  - **Raoul Pal (Real Vision)**: Macro liquidity cycles, generational wealth transfer
  - **Nic Carter (Castle Island Ventures)**: Bitcoin fundamentals, ESG narrative analysis
- **Technical Analysis Experts**:
  - Willy Woo on-chain analysis và network value metrics
  - Plan B Stock-to-Flow model updates và revisions
  - Benjamin Cowen technical analysis và cycle theory
  - Tone Vays traditional technical analysis approach

**🏛️ Academic & Research Institution Insights:**
- **Search keywords**: "MIT Bitcoin research", "Stanford crypto economics", "Harvard Business School Bitcoin", "Federal Reserve crypto research"
- **Academic Research Tracking**:
  - Federal Reserve economic research papers on cryptocurrency
  - MIT Digital Currency Initiative findings
  - Stanford crypto economics research
  - Harvard Business School case studies on crypto adoption
  - University endowment crypto investment research

**📱 Social Media Sentiment - Advanced Analytics:**
- **Cross-check**: Verify social trends với real-time sentiment scoring và price correlation
- **Search keywords**: "crypto Twitter sentiment", "Bitcoin Reddit discussion", "cryptocurrency social sentiment", "fear greed index social media"
- **Platform-Specific Analysis**:
  - **Twitter/X**: Trending hashtags, influencer sentiment, thread engagement metrics
  - **Reddit**: r/cryptocurrency, r/bitcoin comment sentiment, upvote patterns
  - **YouTube**: Creator sentiment analysis, view/engagement trends on crypto content
  - **LinkedIn**: Professional network discussions, institutional adoption sentiment
  - **Telegram**: Community sentiment trong major crypto groups
- **Sentiment Correlation Analysis**:
  - Social sentiment vs Fear & Greed Index correlation
  - Retail sentiment vs institutional flow divergence
  - Geographic sentiment differences (US vs EU vs Asia)
  - Sentiment lag vs price movement analysis

**📊 On-chain Analytics Expert Interpretation:**
- **Cross-check**: Validate on-chain insights với real-time network data và expert commentary
- **Search keywords**: "Glassnode Bitcoin analysis", "CryptoQuant whale alert", "Santiment on-chain metrics", "Whalemap Bitcoin"
- **On-chain Analytics Platforms**:
  - **Glassnode**: Network health, holder behavior, market cycles
  - **CryptoQuant**: Exchange flows, miner behavior, institutional indicators
  - **Santiment**: Social sentiment correlation, developer activity, network growth
  - **Messari**: Protocol fundamentals, tokenomics analysis, competitive landscape
  - **Nansen**: Wallet labeling, smart money tracking, DeFi analytics

**🔮 Contrarian vs Consensus Analysis:**
- **Consensus View Tracking**: Majority opinion từ surveys, polls, và prediction markets
- **Contrarian Indicator Analysis**: Extreme sentiment levels như contrarian signals
- **Prediction Market Insights**: Polymarket, Augur betting odds on crypto events
- **Options Market Sentiment**: Put/call ratios, implied volatility, max pain analysis

#### 1.9 🔮 Kết luận và Triển vọng

**📈 Short-term Outlook (1-4 tuần):**
- **Cross-check**: Integrate tất cả findings với real-time data validation
- Key support và resistance levels cần watch
- Potential catalysts và events có thể impact thị trường
- Risk factors và potential downside scenarios
- Probability-weighted price targets cho major assets

**🚀 Medium-term Perspective (1-3 tháng):**
- **Cross-check**: Align với real-time trend analysis
- Seasonal patterns và historical precedents
- Macro environment evolution và Fed policy timeline
- Institutional adoption trajectory
- Regulatory milestone và potential impacts

**🌟 Long-term Vision (6-12 tháng):**
- **Cross-check**: Validate với real-time adoption metrics
- Technological developments và ecosystem maturation
- Global regulatory framework stabilization
- Institutional infrastructure development
- Market cycle analysis và potential phase transitions

**⚠️ Risk Assessment:**
- **Cross-check**: Monitor real-time risk indicators
- Black swan events và tail risk scenarios
- Correlation breakdown possibilities
- Liquidity crisis potential
- Regulatory crackdown scenarios và mitigation strategies
**Xây dựng các kịch bản có thể xảy ra** 
- Tăng giá cho thị trường trong ngắn hạn (vài ngày đến một tuần tới)
- Đi ngang cho thị trường trong ngắn hạn (vài ngày đến một tuần tới)
- Giảm giá cho thị trường trong ngắn hạn (vài ngày đến một tuần tới)
---

### GIAI ĐOẠN 2: 🔍 SELF-VALIDATION & VERIFICATION

#### 2.1 📊 Bảng Đối chiếu Dữ liệu Real-time

Tạo bảng **"Bảng Đối chiếu Dữ liệu & Nguồn"**:

```markdown
| Dữ liệu / Chỉ số | Giá trị Báo cáo | Giá trị Real-time | Độ lệch | Nguồn | Trạng thái |
|------------------|-----------------|-------------------|---------|-------|------------|
| BTC Price | $X | $Y | Z% | Google Search | ✅/❌ |
| 24h Change | X% | Y% | Z% | CoinMarketCap | ✅/❌ |
| Fear & Greed | X | Y | Z | Alternative.me | ✅/❌ |
```

#### 2.2 ✅ Auto-correction Process

**Tiêu chí validation:**
- BTC Price: Độ lệch ≤ 2% với real-time data
- 24h Change: Độ lệch ≤ 20% with real-time data  
- Fear & Greed Index: Độ lệch ≤ 10% with real-time data

**Nếu phát hiện sai lệch:**
- **Sai lệch nhỏ** (≤ tolerance): Giữ nguyên, note disclaimer
- **Sai lệch lớn** (> tolerance): **TỰ ĐỘNG SỬA LẠI** số liệu trong báo cáo ở GIAI ĐOẠN 1
- **Dữ liệu không khả dụng**: Sử dụng latest available data với timestamp

#### 2.3 🎯 Final Self-Assessment

Sau khi hoàn thành validation và correction, đưa ra kết luận:

**✅ Nếu tất cả dữ liệu accurate (trong tolerance):**
```
KẾT QUẢ KIỂM TRA: PASS
Lý do: Tất cả dữ liệu quan trọng nằm trong ngưỡng chấp nhận với real-time system
```

**❌ Nếu vẫn có dữ liệu sai lệch lớn sau correction:**
```
KẾT QUẢ KIỂM TRA: FAIL
Lý do: [Mô tả cụ thể dữ liệu nào vẫn sai lệch và tại sao không thể correct]
```

---

## 📤 OUTPUT FORMAT

### 🎯 FINAL RESEARCH REPORT

[Báo cáo nghiên cứu đầy đủ đã được auto-corrected]

### 🔍 VALIDATION SUMMARY

[Bảng đối chiếu dữ liệu như trên]

### ✅ SELF-VALIDATION RESULT

**KẾT QUẢ KIỂM TRA: PASS/FAIL**
**Lý do:** [Chi tiết explanation]

---

## 🚨 CRITICAL REQUIREMENTS

1. **Phải complete cả 2 giai đoạn** trong single response
2. **Auto-correction**: Tự động sửa dữ liệu sai lệch trong báo cáo
3. **Format "KẾT QUẢ KIỂM TRA: PASS/FAIL"** bắt buộc để system parsing
4. **Báo cáo final bằng tiếng Việt**
5. **Validation summary bằng tiếng Anh** để consistency


## 📋 Yêu cầu về Nguồn và Dữ liệu

### 🎯 Ưu tiên hàng đầu:
- Lấy dữ liệu mới nhất từ **TradingView** và **CoinMarketCap**
- Cross-validate với **real-time data** từ hệ thống

### 📰 Nguồn tham khảo chính (Tier 1):
- **Tài chính Truyền thống**: Bloomberg, Reuters, Financial Times, Wall Street Journal, MarketWatch
- **Crypto Native**: CoinDesk, The Block, Decrypt, CryptoSlate, Coinbase Research
- **Analytics Platforms**: Glassnode, CryptoQuant, Santiment, Messari, Nansen

### 🔍 Nguồn tham khảo bổ sung (Tier 2):
- **Regulatory**: SEC.gov, CFTC.gov, Federal Reserve publications, Congressional hearings
- **Corporate**: SEC EDGAR filings, earnings call transcripts, corporate press releases
- **Academic**: MIT DCI, Stanford crypto research, Federal Reserve economic papers
- **Social Intelligence**: Twitter verified accounts, Reddit communities, YouTube analytics

### 🌏 Ngôn ngữ:
- Báo cáo cuối cùng: **tiếng Việt**
- Validation summary: **tiếng Anh**

### ⚠️ Chất lượng Nguồn:
- **Bắt buộc verify từ 2+ independent tier 1 sources** cho mọi thông tin quan trọng
- **Timestamp tracking** cho tất cả dữ liệu và tin tức
- **Source attribution** cho mọi claims và predictions
- **Bias assessment** cho expert opinions và institutional research
"#;
