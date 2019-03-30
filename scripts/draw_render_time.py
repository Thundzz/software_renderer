import matplotlib.pyplot as plt 

xs = []
ys = []

with open("log_3.txt") as file:
	lines = file.readlines()
	for line in lines:
		splitted = line.split()
		xs.append(float(splitted[0]))
		ys.append(float(splitted[1]))

plt.plot(xs, ys)
plt.show()




