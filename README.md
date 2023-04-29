# Momentum trading algorithm in Rust

Trades momentum for 10 stocks
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