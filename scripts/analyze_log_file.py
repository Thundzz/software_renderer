import pandas as pd 

files = [
    # "logs_800_600.csv", 
    # "logs_800_600_noloop.csv", 
    # "logs_800_600_noloop_2.csv", 
    # "logs.csv",
    # "nosleep.csv",
    "nopoll.csv"
]

for file in files:
    df = pd.read_csv(file, header=None)
    print(df[df[0] > 5])
