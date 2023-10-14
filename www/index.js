import { Crossmath } from "wasm-crossmath";

function generate_grid() {
    const crossmath = Crossmath.new(6);
    const width = crossmath.width();
    const height = crossmath.height();
    const grid = crossmath.render();
    console.log(width)
    console.log(grid);
    show_grid(grid, width)
}

function show_grid(grid, width) {
    let board = document.querySelector("#board");
    board.innerHTML = "";
    for(const value of grid.split(";").slice(0, -1)) {
        let block = document.createElement("div");
        block.classList.add("block");
        if(value === "?") {
            block.classList.add("hidden");
        } else if(value != " " && value != "\n" && value != "\0") {
            block.classList.add("filled");
            block.textContent = value;
        }
        board.appendChild(block)
    }
    board.style.setProperty("--grid-width", width.toString())
}

generate_grid();