let currentPoints = {};

console.log(document.getElementsByTagName("canvas"));
for (const canvas of document.getElementsByTagName("canvas")) {
    if (canvas.dataset.vec2d === undefined) continue;

    currentPoints[canvas.id] = {
        x: canvas.dataset.vec2dDefaultX,
        y: canvas.dataset.vec2dDefaultY,
    };
    console.log(canvas.dataset)

    let ctx = canvas.getContext("2d");
    ctx.font = "1em sans";

    const w = canvas.width;
    const h = canvas.height;

    const left = 0, right = w, top = 0, bottom = h;

    const canvasX = (x) => ((x - offsetX) + 1) / 2 * w;
    const canvasY = (y) => (-(y - offsetY) + 1) / 2 * h;
    const positionToCanvasX = x => ((x / w) * 2) - 1 + offsetX;
    const positionToCanvasY = y => -((((y) / h) * 2) - 1 - offsetY);

    const offsetX = 0.;
    const offsetY = 0.;

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
    const clearCircle = (x, y, radius) => {
        ctx.save();
        ctx.globalCompositeOperation = 'destination-out';
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, 2 * Math.PI, false);
        ctx.fill();
        ctx.restore();
    };

    const drawCoordinateSystem = () => {

        ctx.clearRect(0, 0, w, h);

        ctx.lineWidth = 2;
        line([left, canvasY(0)], [right, canvasY(0)]);
        line([canvasX(0), top], [canvasX(0), bottom]);


        const roundTowards = (value, towards) => {
            let inv = 1 / towards;
            return Math.round(value * inv) / inv;
        }

        const ticks = (start, end, step, f) => {
            for (value = roundTowards(start, step); value < end; value += step) {
                f(value);
            }
        };

        ctx.lineWidth = 0.5;
        ctx.strokeStyle = "#888";

        ctx.textBaseline = "middle";
        ctx.textAlign = "center";

        ticks(positionToCanvasX(left), positionToCanvasX(right), 0.2, (x) => {
            const cx = canvasX(x);
            line([cx, top], [cx, bottom]);
        });
        // ticks(positionToCanvasX(left), positionToCanvasX(right), 1, (x) => {
        //     text(x.toString(), canvasX(x), canvasY(-0.2));
        // });

        ticks(positionToCanvasY(bottom), positionToCanvasY(top), 0.2, (y) => {
            const cy = canvasY(y);
            line([left, cy], [right, cy]);
        });
        // ticks(positionToCanvasY(bottom), positionToCanvasY(top), 1, (y) => {
        //     text(y.toString(), canvasX(-0.2), canvasY(y));
        // });
        ctx.strokeStyle = "#000";

        /*ctx.textBaseline = "top";
        text("-1", 0.02, 0.52);
        ctx.textAlign = "right";
        text("1", 0.98, 0.52);
        text("-1", 0.48, 0.02);
        ctx.textBaseline = "alphabetic"
        text("-1", 0.48, 0.98);
        ctx.textAlign = "left";*/

        const mouse = currentPoints[canvas.id];
        circle(canvasX(mouse.x), canvasY(mouse.y), 5);
    }

    drawCoordinateSystem();

    canvas.addEventListener("mousemove", (e) => {
        if (e.buttons !== 1) return;

        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;

        const x = positionToCanvasX((e.clientX - rect.left) * scaleX);
        const y = positionToCanvasY((e.clientY - rect.top) * scaleY);

        currentPoints[e.target.id] = { x, y };
        drawCoordinateSystem();

        canvas.dispatchEvent(new CustomEvent("vec2d-data", { detail: { x, y } }));
    });
}
