let currentPoints = {};

function desiredStep(x) {
    // 0.1, 1, 10, 100, etc.
    const nearestExpOf10 = Math.pow(10, Math.floor(Math.log10(x)));
    const selection = [nearestExpOf10, nearestExpOf10 * 2, nearestExpOf10 * 5, nearestExpOf10 * 10]

    const deltas = selection.map(s => Math.abs(x - s));
    const closest = Math.min(...deltas);
    const index = deltas.indexOf(closest);

    return selection[index];
}

function parsePoint(point) {
    let [x, y] = point.split(",");
    return { x: parseFloat(x), y: parseFloat(y) };
}

const ticks = (start, end, step, f) => {
    let firstTick = Math.round(start / step) * step; // round towards step
    for (let value = firstTick; value < end; value += step) {
        value = parseFloat(value.toFixed(6));
        f(value);
    }
};

for (const canvas of document.getElementsByTagName("canvas")) {
    if (canvas.dataset.vec2d === undefined) continue;
    const defaultValue = parsePoint(canvas.dataset.vec2dDefault);

    let min = parsePoint(canvas.dataset.vec2dMin);
    let max = parsePoint(canvas.dataset.vec2dMax);

    const width = () => max.x - min.x;
    const height = () => max.y - min.x;

    currentPoints[canvas.id] = defaultValue;

    let ctx = canvas.getContext("2d");
    ctx.font = "0.8em sans";
    const line = (from, to) => {
        ctx.beginPath();
        ctx.moveTo(from[0], from[1]);
        ctx.lineTo(to[0], to[1]);
        ctx.stroke();
    }
    const text = (text, x, y) => {
        ctx.fillText(text, x, y);
    }
    const circle = (x, y, radius) => {
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2, false);
        ctx.fill();
    }

    const canvasWidth = canvas.width;
    const canvasHeight = canvas.height;

    const canvasScaleX = () => canvasWidth / width();
    const canvasScaleY = () => canvasHeight / height();

    const left = 0, right = canvasWidth, top = 0, bottom = canvasHeight;

    let isDragging = false;
    let currentDrag = { x: 0, y: 0 };
    let dragStart = { x: 0, y: 0 };

    // from [min.x, max.x] to [0, canvasWidth]
    const canvasX = x => ((x - currentDrag.x) - min.x) / (max.x - min.x) * canvasWidth;
    // from [min.y, max.y] to [canvasHeight, 0]
    const canvasY = y => ((y - currentDrag.y) - max.y) / (min.y - max.y) * canvasHeight;
    // [0, canvasWidth] to [min.x, max.x]
    const positionToCanvasX = x => (x / canvasWidth * (max.x - min.x) + min.x) + currentDrag.x;
    // [canvasHeight, 0] to [min.y, max.y
    const positionToCanvasY = y => (y / canvasHeight * (min.y - max.y) + max.y) + currentDrag.y;

    const positionFromCanvasEvent = (canvas, e, withDragOffset = true) => {
        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;

        let x = positionToCanvasX((e.clientX - rect.left) * scaleX);
        let y = positionToCanvasY((e.clientY - rect.top) * scaleY);

        if (!withDragOffset) {
            x -= currentDrag.x;
            y -= currentDrag.y;
        }

        return { x, y }
    }

    const drawCoordinateSystem = () => {
        ctx.clearRect(0, 0, canvasWidth, canvasHeight);

        ctx.lineWidth = 2;
        line([left, canvasY(0)], [right, canvasY(0)]);
        line([canvasX(0), top], [canvasX(0), bottom]);

        const tickStepX = desiredStep(width() / 8);
        const tickStepY = desiredStep(height() / 8);

        ctx.strokeStyle = "#888";
        ctx.lineWidth = 0.5;
        // vertical grid lines
        ticks(positionToCanvasX(left), positionToCanvasX(right), tickStepX, (x) => {
            const cx = canvasX(x);
            line([cx, top], [cx, bottom]);
        });
        // horizontal grid lines
        ticks(positionToCanvasY(bottom), positionToCanvasY(top), tickStepY, (y) => {
            const cy = canvasY(y);
            line([left, cy], [right, cy]);
        });

        ctx.strokeStyle = "#000";
        ctx.lineWidth = 1;

        ctx.textAlign = "center";
        ctx.textBaseline = "alphabetic";

        // x-axis ticks
        ticks(positionToCanvasX(left), positionToCanvasX(right), tickStepX, (x) => {
            let extentY = (tickStepY / 4) * canvasScaleY();
            let cx = canvasX(x);
            let cy0 = canvasY(0);
            line([cx, cy0 - extentY], [cx, cy0 + extentY]);

            if (x !== 0) text(x.toString(), canvasX(x), canvasY(0) - extentY * 1.5);
        });

        ctx.textAlign = "right";
        ctx.textBaseline = "middle";
        // y-axis ticks
        ticks(positionToCanvasY(bottom), positionToCanvasY(top), tickStepY, (y) => {
            let extentX = (tickStepX / 4) * canvasScaleX();
            let cy = canvasY(y);
            let cx0 = canvasX(0);
            line([cx0 - extentX, cy], [cx0 + extentX, cy]);

            if (y !== 0) text(y.toString(), canvasX(0) - extentX * 1.5, canvasY(y));
        });

        const mouse = currentPoints[canvas.id];
        circle(canvasX(mouse.x), canvasY(mouse.y), 5);
    }

    drawCoordinateSystem();

    const beginDrag = (e) => {
        const position = positionFromCanvasEvent(canvas, e, false);
        isDragging = true;
        dragStart = position;
    }

    const endDrag = () => {
        isDragging = false;
        dragStart = { x: 0, y: 0 };
    }

    const handleDrag = (e) => {
        const mousePoint = positionFromCanvasEvent(canvas, e, false);
        let diff = { x: dragStart.x - mousePoint.x, y: dragStart.y - mousePoint.y }
        currentDrag.x += diff.x;
        currentDrag.y += diff.y;
        dragStart = mousePoint;

        drawCoordinateSystem();
    };

    const handlePointSelection = (e) => {
        const position = positionFromCanvasEvent(canvas, e);

        currentPoints[e.target.id] = position;
        drawCoordinateSystem();
        canvas.dispatchEvent(new CustomEvent("vec2d-data", { detail: position }));
    };

    const zoom = (e) => {
        e.preventDefault();

        const factor = 1 + e.deltaY * 0.05;
        const { x, y } = positionFromCanvasEvent(canvas, e);

        // the percentage of the point from start to end stays the same (I)
        // and sizeBefore * factor = sizeAfter (II);
        // Applies for both x and y;

        // I:  (x-minBefore) / (maxBefore-minBefore) = (x-minAfter) / (maxAfter-minAfter)
        // II: (maxBefore-minBefore)*factor = (maxAfter-minAfter)
        // WolframAlpha: solve((x-a)/(b-a)=(x-g)/(h-g), (b-a)*f=(h-g), g, h)
        // g = a f - f x + x and h = b f - f x + x and a f!=b f

        min.x = min.x * factor - factor * x + x;
        max.x = max.x * factor - factor * x + x;

        min.y = min.y * factor - factor * y + y;
        max.y = max.y * factor - factor * y + y;

        drawCoordinateSystem();
    };

    canvas.addEventListener("mousemove", (e) => {
        if (isDragging) {
            handleDrag(e);
        } else if (e.buttons === 1) {
            handlePointSelection(e);
        }
    });
    canvas.addEventListener("mousedown", (e) => {
        if (e.shiftKey) beginDrag(e);
        else handlePointSelection(e);
    });

    canvas.addEventListener("mouseup", endDrag);
    canvas.addEventListener("mouseout", endDrag);

    canvas.addEventListener("wheel", zoom);
}
