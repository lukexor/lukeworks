import p5 from "p5";
import { awaitClickStart } from "../utils";

const mazeAstarSketch = (p: p5) => {
  const cellSize = p.windowWidth / 30;
  const canvasWidth = p.windowWidth;
  const canvasHeight = cellSize * 15;

  let createMazeBtn: p5.Element;
  let solveMazeBtn: p5.Element;
  let msg: p5.Element;

  const connectChance = 10;

  let cols: number;
  let rows: number;
  let grid: Cell[] = [];

  let stack: Cell[] = [];
  let current: Cell | undefined;
  let cellsVisited: number;
  let mazeGenerated = false;

  // A*
  let openSet: MinHeap;
  let closedSet: Record<number, boolean>;
  let start: Cell;
  let end: Cell;
  let path: Cell[];
  let pathSet: Record<number, Cell>;

  let backtracking = false;
  let startTime = Date.now();

  p.disableFriendlyErrors = true;

  p.setup = () => {
    p.createCanvas(canvasWidth, canvasHeight);
    p.textSize(16);
    p.fill(255);

    awaitClickStart(p, undefined, () => {
      createGrid();

      createMazeBtn = p.createButton("Create Maze");
      createMazeBtn.style("margin", "5px");
      createMazeBtn.mousePressed(() => createMaze());

      solveMazeBtn = p.createButton("Solve Maze");
      solveMazeBtn.style("margin", "5px");
      solveMazeBtn.mousePressed(() => solveAstar());

      msg = p.createP("");
      msg.style("padding", "0 5px");
    });
  };

  const createGrid = () => {
    grid = [];
    stack = [];
    cellsVisited = 0;
    cols = p.floor(p.width / cellSize);
    rows = p.floor(p.height / cellSize);
    for (let row = 0; row < rows; ++row) {
      for (let col = 0; col < cols; ++col) {
        grid.push(new Cell(col, row));
      }
    }
    p.background(51);
    grid.forEach((cell) => cell.draw());
  };

  const startCreateMaze = () => {
    msg.elt.innerHTML =
      "Creating a maze using a Recursive Backtracking algorithm...<br /><br />" +
      "Green: Current Position<br />" +
      "Gray: Unvisited Cell<br />" +
      "Blue: Visited Cell";
    createGrid();
  };

  const finishCreateMaze = () => {
    const elapsedStr = elapsed();
    msg.elt.innerHTML =
      `Finished in ${elapsedStr}. Click 'Solve Maze' to generate random ` +
      "start/end points and solve using an A* search algorithm.";
    mazeGenerated = true;
    current = grid[0];
  };

  const createMazeDraw = () => {
    p.background(51);
    grid.forEach((cell) => cell.draw());

    if (!current) {
      return;
    }

    if (!current.visited) {
      cellsVisited += 1;
    }
    current.visited = true;
    current.highlight();
    const next = current.getRandomNeighbor();
    if (next) {
      stack.push(next);
      current.removeWallTo(next);
      current = next;
      backtracking = false;
    } else if (stack.length > 0) {
      if (!backtracking) current.connectRandomNeighbor();
      backtracking = true;
      current = stack.pop();
    } else if (cellsVisited === grid.length) {
      finishCreateMaze();
      grid.forEach((cell) => cell.draw());
      p.noLoop();
    }
  };

  const createMaze = () => {
    startCreateMaze();
    const cell = grid[p.floor(p.random() * grid.length)];
    if (cell) {
      mazeGenerated = false;
      current = cell;
      stack.push(current);
      startTime = Date.now();
      p.draw = createMazeDraw;
      p.loop();
    }
  };

  const createInstantMaze = () => {
    startCreateMaze();
    const cell = grid[p.floor(p.random() * grid.length)];
    if (cell) {
      current = cell;
      startTime = Date.now();
      stack.push(current);
      mazeGenerated = false;

      while (!mazeGenerated && current) {
        if (!current.visited) cellsVisited += 1;
        current.visited = true;
        const next = current.getRandomNeighbor();
        if (next) {
          stack.push(next);
          current.removeWallTo(next);
          current = next;
          backtracking = false;
        } else if (stack.length > 0) {
          if (!backtracking) current.connectRandomNeighbor();
          backtracking = true;
          current = stack.pop();
        } else if (cellsVisited === grid.length) {
          finishCreateMaze();
        }
      }

      p.draw = () => {
        p.background(51);
        grid.forEach((cell) => cell.draw());
        setTimeout(() => {
          p.noLoop();
        }, 100);
      };

      p.loop();
    }
  };

  const solveAstarDraw = () => {
    p.background(51);

    path = [];
    pathSet = {};
    let temp = current;
    if (temp) {
      path.push(temp);
    }
    while (temp && temp.previous) {
      path.push(temp.previous);
      pathSet[temp.id] = temp;
      temp = temp.previous;
    }

    grid.forEach((cell) => {
      if (pathSet[cell.id]) {
        cell.draw([0, 125, 125]);
      } else if (closedSet[cell.id]) {
        cell.draw([125, 0, 0]);
      } else if (openSet.contains(cell)) {
        cell.draw([225, 125, 0]);
      } else {
        cell.draw();
      }
    });

    start.highlight();
    end.draw([255, 255, 0]);

    if (!openSet.isEmpty()) {
      current = openSet.extractMin();

      if (current === end) {
        const elapsedStr = elapsed();
        msg.elt.innerHTML =
          `Finished in ${elapsedStr}. Click 'Solve Maze' to choose different ` +
          "starting/exit points, or 'Create Maze' to generate a new maze.";
        openSet.clear();
        solveAstarDraw();
        p.noLoop();
        return;
      }

      if (current) {
        closedSet[current.id] = true;

        current.neighbors.forEach((neighborId) => {
          if (neighborId) {
            const neighbor = grid[neighborId];
            if (neighbor && current && !closedSet[neighbor.id]) {
              const tempG = current.g + 1;
              if (tempG < neighbor.g) {
                neighbor.previous = current;
                neighbor.g = tempG;
                neighbor.h = neighbor.heuristic(end);
                neighbor.f = neighbor.g + neighbor.h;
                if (!openSet.contains(neighbor)) {
                  openSet.insert(neighbor);
                }
              }
            }
          }
        });
      }
    } else {
      p.noLoop();
    }
  };

  const solveAstar = () => {
    if (!mazeGenerated) {
      createInstantMaze();
    }
    msg.elt.innerHTML =
      "Solving the maze using an A* algorithm...<br /><br />" +
      "Green: Start -- Yellow: Goal<br />" +
      "Orange: Possible Paths<br />" +
      "Red: Explored Paths<br />" +
      "Cyan: Shortest Path";
    openSet = new MinHeap();
    closedSet = {};
    path = [];
    pathSet = {};
    grid.forEach((cell) => {
      cell.previous = null;
      cell.f = cell.g = cell.h = Number.MAX_VALUE;
    });
    const randomStart = grid[p.floor(p.random() * grid.length)];
    const randomEnd = grid[p.floor(p.random() * grid.length)];
    if (randomStart && randomEnd) {
      start = randomStart;
      end = randomEnd;
      current = start;
      start.g = 0;
      start.h = start.heuristic(end);
      start.f = start.h;
      openSet.insert(start);
      startTime = Date.now();
      p.draw = solveAstarDraw;
      p.loop();
    }
  };

  const index = (x: number, y: number) => {
    if (x < 0 || x > cols - 1 || y < 0 || y > rows - 1) return -1;
    return x + y * cols;
  };

  class Cell {
    id: number;
    col: number;
    row: number;
    visited = false;
    neighbors: (null | number)[];
    previous: null | Cell = null;
    f: number;
    g: number;
    h: number;

    constructor(col: number, row: number) {
      this.id = grid.length;
      this.col = col;
      this.row = row;
      this.neighbors = [null, null, null, null];

      // A*
      this.f = Number.MAX_VALUE;
      this.g = Number.MAX_VALUE;
      this.h = Number.MAX_VALUE;
    }

    draw(color: number[] = [0, 50, 75]) {
      const top = this.row * cellSize;
      const left = this.col * cellSize;
      const right = left + cellSize;
      const bottom = top + cellSize;

      p.push();
      if (this.visited) {
        p.fill(color);
        p.noStroke();
        p.rect(left, top, cellSize + 1, cellSize + 1);
      }
      p.noFill();
      p.stroke(255);
      if (this.neighbors[0] === null) p.line(left, top, right, top); // Top
      if (this.neighbors[1] === null) p.line(right, top, right, bottom); // Right
      if (this.neighbors[2] === null) p.line(right, bottom, left, bottom); // Bottom
      if (this.neighbors[3] === null) p.line(left, bottom, left, top); // Left
      p.pop();
    }

    getRandomNeighbor() {
      const neighbors = [];
      const top = grid[index(this.col, this.row - 1)];
      const right = grid[index(this.col + 1, this.row)];
      const bottom = grid[index(this.col, this.row + 1)];
      const left = grid[index(this.col - 1, this.row)];
      if (top && !top.visited) neighbors.push(top);
      if (right && !right.visited) neighbors.push(right);
      if (bottom && !bottom.visited) neighbors.push(bottom);
      if (left && !left.visited) neighbors.push(left);

      if (neighbors.length > 0) {
        const r = p.floor(p.random() * neighbors.length);
        return neighbors[r];
      } else {
        return undefined;
      }
    }

    connectRandomNeighbor() {
      const chance = p.floor(p.random() * 100);
      if (chance < connectChance) {
        const newNeighbors = [];
        const top = grid[index(this.col, this.row - 1)];
        const right = grid[index(this.col + 1, this.row)];
        const bottom = grid[index(this.col, this.row + 1)];
        const left = grid[index(this.col - 1, this.row)];
        if (top && !this.neighbors.includes(top.id)) newNeighbors.push(top);
        if (right && !this.neighbors.includes(right.id))
          newNeighbors.push(right);
        if (bottom && !this.neighbors.includes(bottom.id))
          newNeighbors.push(bottom);
        if (left && !this.neighbors.includes(left.id)) newNeighbors.push(left);

        if (newNeighbors.length > 0) {
          const remove =
            newNeighbors[p.floor(p.random() * newNeighbors.length)];
          if (remove) {
            this.removeWallTo(remove);
          }
        }
      }
    }

    removeWallTo(cell: Cell) {
      const x = this.col - cell.col;
      if (x === 1) {
        this.neighbors[3] = grid[index(this.col - 1, this.row)]?.id ?? null; // Left
        cell.neighbors[1] = this.id;
      } else if (x === -1) {
        this.neighbors[1] = grid[index(this.col + 1, this.row)]?.id ?? null; // Right
        cell.neighbors[3] = this.id;
      }

      const y = this.row - cell.row;
      if (y === 1) {
        this.neighbors[0] = grid[index(this.col, this.row - 1)]?.id ?? null; // Top
        cell.neighbors[2] = this.id;
      } else if (y === -1) {
        this.neighbors[2] = grid[index(this.col, this.row + 1)]?.id ?? null; // Bottom
        cell.neighbors[0] = this.id;
      }
    }

    heuristic(cell: Cell) {
      const a = this.col - cell.col;
      const b = this.row - cell.row;
      return Math.hypot(a, b);
    }

    highlight() {
      const left = this.col * cellSize;
      const top = this.row * cellSize;
      p.fill(0, 155, 0);
      p.noStroke();
      p.rect(left, top, cellSize, cellSize);
    }
  }

  class MinHeap {
    heap: Cell[];
    set: Record<number, boolean>;

    constructor() {
      this.heap = [];
      this.set = {};
    }

    parent(i: number) {
      return p.floor((i - 1) / 2);
    }

    left(i: number) {
      return 2 * i + 1;
    }

    right(i: number) {
      return 2 * i + 2;
    }

    swap(i: number, j: number) {
      const a = this.heap[i];
      const b = this.heap[j];
      if (a && b) {
        [this.heap[i], this.heap[j]] = [b, a];
      }
    }

    isEmpty() {
      return this.heap.length === 0;
    }

    clear() {
      this.heap = [];
      this.set = {};
    }

    getF(i: number) {
      return this.heap[i]?.f ?? 0;
    }

    getMin() {
      return this.heap[0];
    }

    contains(cell: Cell) {
      return this.set[cell.id];
    }

    insert(cell: Cell) {
      this.heap.push(cell);
      this.set[cell.id] = true;
      let i = this.heap.length - 1;
      while (i > 0 && this.getF(this.parent(i)) > this.getF(i)) {
        this.swap(i, this.parent(i));
        i = this.parent(i);
      }
    }

    heapify(i: number) {
      const l = this.left(i);
      const r = this.right(i);
      let min = i;
      const size = this.heap.length;
      if (l < size && this.getF(l) < this.getF(min)) min = l;
      if (r < size && this.getF(r) < this.getF(min)) min = r;
      if (min != i) {
        this.swap(i, min);
        this.heapify(min);
      }
    }

    extractMin() {
      const size = this.heap.length;
      if (size == 0) {
        return undefined;
      } else if (size == 1) {
        this.set = {};
        return this.heap.pop();
      }
      const root = this.heap[0];
      if (root) {
        delete this.set[root.id];
      }
      const last = this.heap.pop();
      if (last) {
        this.heap[0] = last;
        this.heapify(0);
      }
      return root;
    }
  }

  const elapsed = () => {
    const time = Date.now() - startTime;
    const ms = ("00" + (time % 1000)).slice(-3);
    const sec = ("0" + p.round(time / 1000)).slice(-2);
    const min = ("0" + p.round(time / (60 * 1000))).slice(-2);
    const hours = ("0" + p.round(time / (60 * 60 * 1000))).slice(-2);
    return `${hours}:${min}:${sec}:${ms}`;
  };
};

export default mazeAstarSketch;
