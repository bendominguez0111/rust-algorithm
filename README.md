# Momentum trading algorithm in Rust

Trades momentum for 9 different stocks in 9 different industries and 1 equity index (SPY).

Symbols traded below:

AAPL: Apple (Consumer Goods)
AAL: American Airlines (Airlines)
F: Ford Motors (Auto Manufacturers)
HD: Home Depot (Consumer Durables)
INTC: Intel Corporation (Semis)
JPM: JP Morgan Chase (Retail & Institutional Banking)
META: Meta Platforms (Tech)
SPY: S&P 500 (Equity Index)
V: Visa (Financials)
XOM: Exxon Mobil (Energy)

Equal portfolio weighting for now but eventually will weight based off abs(% spread in short and long EMA) / standard deviation.


[1, 4, 5, 6, 7, 9, 10] # spreads

[1, 2, 3, 5 , 9, 8, 10] # standard deviations

["SPY", "META", "F", "AAPL", "INTC", "XOM", "JPM", "HD", "V", "AAL"] #symbols

[9.52, 19.05, 16.19, 11.43, 7.62, 10.48, 9.52] # portfolio weights


200% margin


Long SPY for $9,400 for $100,000 net liquidating value

9400/100000 = 9.40%

increase allocation => 9.52%


100,000 +- 1,000
90,000 +- 900
110,000 +- 1,100