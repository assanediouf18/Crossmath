import { Crossmath } from "wasm-crossmath";

function generate_grid() {
    const crossmath = Crossmath.new(6);
    const width = crossmath.width();
    const height = crossmath.height();
    const grid = crossmath.render();

    document.querySelector('#checkBtn').addEventListener("click", () => {
        let answer = build_answer();
        console.log(answer);
        let val = crossmath.check(answer);
        console.log(val);
    })

    show_grid(grid, width, height)
}

function show_grid(grid, width, height) {
    let board = document.querySelector("#board");
    board.innerHTML = "";
    let grid_len = width * height;
    for(const value of grid.split(";").slice(0, grid_len)) {
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

    let options = document.querySelector("#options")
    for(const value of grid.split(";").slice(grid_len, -1)) {
        let block = document.createElement("div");
        block.classList.add("block");
        block.classList.add("option");
        block.textContent = value;
        options.appendChild(block)
    }
}

function build_answer() {
    let board = document.querySelector("#board");
    let answer = "";
    for(const block of board.children) {
        answer += (block.textContent) ? block.textContent : " ";
        answer += ";"
    }
    return answer;
}

generate_grid();