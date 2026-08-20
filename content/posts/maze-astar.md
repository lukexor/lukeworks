---
title: "Maze Generation and A* Search"
kind: project
description: A maze generator and an A-star solver, drawn as they run.
technologies: [JavaScript, p5.js]
image:
  src: /images/projects/maze-astar.webp
  alt: "Maze A* Search"
website: /sketch/maze-astar
published: 2020-02-01T19:57:51Z
started: 2020-01-28T19:57:34Z
completed: 2020-01-29T18:16:02Z
updated: 2020-02-01T19:57:57Z
---

<iframe
  title="Maze A* Search"
  src="/sketch/maze-astar"
  width="100%"
  frameborder="0"
  height="500px"
  scrolling="no"
></iframe>

Lately, I've been having a lot of fun putting this maze generator/A* search
program together. It's really enjoyable visualizing the algorithms as they loop
through each iteration. So much so that I'm going to be doing a whole series of
these based on various algorithms and data structures. My initial working
implementation was using plain Javascript and canvas, but I've updated it to use
the [ps5.js][] library which will be useful later on when I get to some more
advanced algorithms.

The program allows you to test various settings for maze generation including
cell size and how often the algorithm will connect through when it encounters a
dead-end and has to backtrack. I added the ability to download and upload saved
mazes since the smaller cell sizes can take quite a long time to generate and I
wanted to be able to test A* against them (and hopefully some other search
algorithms soon) later on. Once a maze is generated, each time you click *Solve
Maze* it will generate random starting and goal points. You can also
pause/unpause the algorithm as well as adjust the frame rate for how fast it
updates.

Keep an eye out for more updates soon!

Huge shout out to [The Coding Train][] for getting me interested in visual
algorithms and ps5.js!

[ps5.js]: https://p5js.org/
[The Coding Train]: https://www.youtube.com/channel/UCvjgXvBlbQiydffZU7m1_aw
