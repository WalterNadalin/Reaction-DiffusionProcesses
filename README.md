# Stochastic simulations of reaction-diffusion processes
In this educational project I will attempt to explore and better learn the theory and methods behind *stochastic simulation of reaction-diffusion processes*. 

This work is (almost) entirely based on what has already been done in [[1]](#PG). In the explanations (again, heavily drawn from [[1]](#PG)) I will write some comments on probability, information theory, and statistical mechanics, topics I am currently studying. **Warning**: they may be completely wrong.

The article is divided into the following sections (which follow the same structure as the article mentioned above). There will also be some `C++` and `python` codes that implement the algorithms and help visualize the results.

## Stochastic simulation of chemical reactions
Exploration of stochastic methods for the modelling of (spatially homogeneous) **systems of chemical reactions** through the following examples.

### Stochastic simulation of degradation
Consider the **single chemical reaction** of *degradation*

$$
A\overset{k}{\rightarrow}\emptyset
$$

where
- $A$ is the *chemical species* of interest
- $k$ is the constant *rate* of the reaction
- $\emptyset$ denotes chemical species which are of *no further interest* in what follows

In order to simulate it, it is used firstly a *naive* stochastic simulation algorithm (**SSA**) and then an improvement of it.
 
## References
<a id="PG">[1]</a> 
Erban, Radek and Chapman, Jonathan and Maini, Philip (2007).
*A practical guide to stochastic simulations of reaction-diffusion processes*.
arXiv.
[10.48550/ARXIV.0704.1908](https://arxiv.org/abs/0704.1908).
