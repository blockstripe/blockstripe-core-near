function Logo(htmlId) {
    const elementContainer = document.getElementById(htmlId);
    let logo = document.getElementById('logo-canvas');

    if (!!logo) {
        console.log('logo already added!');

        main(logo);

        return;
    }

    logo = document.createElement('canvas');

    logo.setAttribute('id', 'logo-canvas');

    elementContainer.appendChild(logo);

    main(logo);
}

function main(logoElement) {
    const webGL_2 = logoElement.getContext('webgl2');

    if (!webGL_2) {
        console.log('webgl2 is not supported!');

        return;
    }

    var vertexShaderSource = `#version 300 es

    // an attribute is an input (in) to a vertex shader.
    // It will receive data from a buffer
    in vec2 a_position;

    // Used to pass in the resolution of the canvas
    uniform vec2 u_resolution;

    // all shaders have a main function
    void main() {

    // convert the position from pixels to 0.0 to 1.0
    vec2 zeroToOne = a_position / u_resolution;

    // convert from 0->1 to 0->2
    vec2 zeroToTwo = zeroToOne * 2.0;

    // convert from 0->2 to -1->+1 (clipspace)
    vec2 clipSpace = zeroToTwo - 1.0;

    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    }
    `;

    var fragmentShaderSource = `#version 300 es

    precision highp float;

    uniform vec4 u_color;

    // we need to declare an output for the fragment shader
    out vec4 outColor;

    void main() {
    outColor = u_color;
    }
    `;

    const program = webglUtils.createProgramFromSources(webGL_2, [vertexShaderSource, fragmentShaderSource]);

    const positionAttributeLocation = webGL_2.getAttribLocation(program, "a_position");
    const resolutionUniformLocation = webGL_2.getUniformLocation(program, "u_resolution");
    const colorLocation = webGL_2.getUniformLocation(program, "u_color");
    const positionBuffer = webGL_2.createBuffer();
    const vertexArr = webGL_2.createVertexArray();

    webGL_2.bindVertexArray(vertexArr);
    webGL_2.enableVertexAttribArray(positionAttributeLocation);
    webGL_2.bindBuffer(webGL_2.ARRAY_BUFFER, positionBuffer);

    const size = 2;
    const type = webGL_2.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;

    webGL_2.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset);

    const translation = [0, 0];
    const width = 50;
    const height = 50;

    drawScene();
    
    function drawScene() {
        const color = [Math.random(), Math.random(), Math.random(), 1];

        webglUtils.resizeCanvasToDisplaySize(webGL_2.canvas);

        webGL_2.viewport(0, 0, webGL_2.canvas.width, webGL_2.canvas.height);
        webGL_2.clearColor(0, 0, 0, 0);
        webGL_2.clear(webGL_2.COLOR_BUFFER_BIT | webGL_2.DEPTH_BUFFER_BIT);
        webGL_2.useProgram(program);
        webGL_2.bindVertexArray(vertexArr);
        webGL_2.uniform2f(resolutionUniformLocation, webGL_2.canvas.width, webGL_2.canvas.height);
        webGL_2.bindBuffer(webGL_2.ARRAY_BUFFER, positionBuffer);

        drawRectangle(webGL_2, translation[0], translation[1], width, height);
    
        webGL_2.uniform4fv(colorLocation, color);
    
        const primitiveType = webGL_2.TRIANGLES;
        const offset = 0;
        const count = 6;

        webGL_2.drawArrays(primitiveType, offset, count);
    }

    function drawRectangle(webGL_2, x, y, width, height) {
        const x1 = x;
        const x2 = x + width;
        const y1 = y;
        const y2 = y + height;

        webGL_2.bufferData(webGL_2.ARRAY_BUFFER, new Float32Array([
            x1, y1,
            x2, y1,
            x1, y2,
            x1, y2,
            x2, y1,
            x2, y2,
        ]), webGL_2.STATIC_DRAW);
    }

    function drawText() {
        
    }

    setInterval(() => {
        drawScene();
    }, 5000);
    /**
     *     function updatePosition(index) {
        return function(_, ui) {
          translation[index] = ui.value;
    
          drawScene();
        };
    }
     */
}

export default Logo;
