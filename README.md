# `yew-boids`

[**View online here :link:**](https://jo12bar.github.io/yew-boids)

My implementation of [Yew's `boids` example app][yew-boids-orig-link], with several small modifications.
Notably, I've modified it to run on a `requestAnimationFrame()` loop instead of a `setInterval()` loop.
This dramatically increases the framerate at the cost of destroying your computer (depending on what
browser you're using - the boids are being rendered as SVG polygons, which can be slow).

[yew-boids-orig-link]: https://github.com/yewstack/yew/tree/0fa83e7dd1c21f892c9c01aaa6dacb073c12a236/examples/boids
