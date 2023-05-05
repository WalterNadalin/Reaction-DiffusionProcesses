### Stochastic simulation of degradation
It is assumed that there are $n_0$ molecules of $A$ in the system at time $t = 0$. The first goal is to compute the number of molecules $A(t)$ for times $t > 0$. 

Given the the things discussed in the [introduction](../README.md###Stochastic_simulation_of_degradation), the probability that exactly one reaction occurs during the infinitesimal time interval $\[t, t + dt)$ is equal to $A(t)kdt$. 

#### Naïve SSA algorithm
Choose a small time step $\Delta t$. Compute the number of molecules $A(t)$ at times $t = i\Delta t$ with $i = 1, 2, 3, \dots$ as follows. Starting with $t = 0$ and $A(0) = n_0$, perform two steps at time $t$:
1. generate a random number $r$ uniformly distributed in the interval $(0, 1)$
2. if $r < A(t)k \Delta t$, then put $A(t + ∆t) = A(t) − 1$

