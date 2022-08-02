<iframe
  title="Fluid Simulation"
  src="https://lukeworks.tech/sketch/fluid-simulation"
  frameborder="0"
  width="100%"
  height="500px"
  scrolling="no"
></iframe>

Fluid Simulation in Javascript using [p5js][]. You can click/touch and drag to
create a fire trail. Inspired by [The Coding Train][]. Also check out this great
[fluid simulation][] article about fluid simulations by Mike Ash. It's amazing
how he broke down the logic and math. Of course, with Javascript we're limited
in processing power, so it's not very high resolution but still really cool! It
works by depositing "dye" with a density value at a certain (x, y) grid position
and then adding some velocity to that area. Then each draw cycle, it performs a
series of diffusion, projection, and advection calculations for each grid
position, accounting for neighboring densities and velocities, solving linear
equations based on the Navier-Stokes equations (which I do not understand). In
this simplistic simulation, it's only simulating in 2D, while the equations over
on Mike Ash's page are for 3D. Also, it's only simulating incompressible fluid
dynamics here. There are a ton of knobs and settings that can be tweaked to get
some really interesting effects!

One major problem I have and still can't figure out how to resolve is that at
the point of "dye" deposit, there are some flickering pixels that are darker
than they should be. I believe it has something to do with velocities being too
high and the advection affecting them in an adverse way. Mike's article links to
this [fire simulation][] that is really neat. That one is much higher resolution
and doesn't suffer from the brightness issue I have. Maybe with some more
tweaking I can get a nicer result but I'm pretty happy about this as it is!

[p5js]: https://p5js.org/
[The Coding Train]: https://www.youtube.com/watch?v=alhpH6ECFvQ
[fluid simulation]: https://mikeash.com/pyblog/fluid-simulation-for-dummies.html
[fire simulation]: https://www.escapemotions.com/experiments/fluid_fire_3/index.php
