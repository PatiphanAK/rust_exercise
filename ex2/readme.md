# Quadratic Equation Solver in Rust

## Description

This program solves a quadratic equation of the form:

\[
ax^2 + bx + c = 0
\]

Given the coefficients \( a \), \( b \), and \( c \), it computes the two roots of the equation using the quadratic formula:

\[
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
\]

If the discriminant (\( b^2 - 4ac \)) is less than zero, the program returns `(0.0, 0.0)`, indicating no real roots.