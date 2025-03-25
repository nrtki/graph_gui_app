const { invoke } = window.__TAURI__.core;

async function loadGraph() {
    const [loadedNodes, loadedEdges] = await invoke('get_graph');
    drawGraph(loadedNodes, loadedEdges);
}

async function addNodeToGrid() {
    const gridContainer = document.querySelector('.grid-container');
    const x = Math.random() * gridContainer.clientWidth;
    const y = Math.random() * gridContainer.clientHeight;

    const node = await invoke('add_node', { x, y });
    console.log('ノードが追加されました:', node);
    loadGraph(); // Re-fetch the graph to get the updated data
}

async function addEdgeToGrid() {
    const sourceIdInput = document.getElementById("source-id");
    const targetIdInput = document.getElementById("target-id");

    const sourceId = sourceIdInput.value;
    const targetId = targetIdInput.value;

    if (sourceId === "" || targetId === "") {
        alert("Please enter both source and target node IDs.");
        return;
    }

    const source = parseInt(sourceId);
    const target = parseInt(targetId);

    if (isNaN(source) || isNaN(target)) {
        alert("Invalid node IDs. Please enter numbers.");
        return;
    }

    try {
        const edge = await invoke('add_edge', { source, target });
        console.log('エッジが追加されました:', edge);
        loadGraph();
        sourceIdInput.value = "";
        targetIdInput.value = "";
    } catch (error) {
        alert(error);
    }
}

function drawGraph(nodes, edges) {
    const gridContainer = document.querySelector('.grid-container');
    // Clear existing elements
    gridContainer.innerHTML = '';

    // Create SVG container
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.style.width = '100%';
    svg.style.height = '100%';
    svg.style.position = 'absolute';
    svg.style.top = '0';
    svg.style.left = '0';
    gridContainer.appendChild(svg);

    // Draw edges
    drawEdges(svg, nodes, edges);
    // Draw nodes
    drawNodes(gridContainer, nodes);
}

function drawEdges(svg, nodes, edges) {
    edges.forEach(edge => {
        const sourceNode = nodes.find(n => n.id === edge.source);
        const targetNode = nodes.find(n => n.id === edge.target);

        if (sourceNode && targetNode) {
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('x1', sourceNode.x + 10); // Adjust for node radius
            line.setAttribute('y1', sourceNode.y + 10);
            line.setAttribute('x2', targetNode.x + 10);
            line.setAttribute('y2', targetNode.y + 10);
            line.setAttribute('stroke', 'black');
            line.setAttribute('stroke-width', '2');
            line.dataset.id = edge.id;
            svg.appendChild(line);
        }
    });
}

function drawNodes(gridContainer, nodes) {
    nodes.forEach(node => {
        const nodeElement = document.createElement('div');
        nodeElement.classList.add('node');
        nodeElement.style.left = node.x + 'px';
        nodeElement.style.top = node.y + 'px';
        nodeElement.dataset.id = node.id;
        gridContainer.appendChild(nodeElement);

        // Style
        nodeElement.style.border = '2px solid black';
        nodeElement.style.backgroundColor = 'white';
        nodeElement.style.width = '20px';
        nodeElement.style.height = '20px';
        nodeElement.style.borderRadius = '50%';
        nodeElement.style.position = 'absolute';
        nodeElement.style.display = 'flex'; // Use flexbox for centering
        nodeElement.style.alignItems = 'center'; // Center vertically
        nodeElement.style.justifyContent = 'center'; // Center horizontally

        // Node ID Label
        const idLabel = document.createElement('span');
        idLabel.textContent = node.id;
        idLabel.style.color = 'black'; // Adjust text color as needed
        idLabel.style.fontSize = '12px'; // Adjust font size as needed
        nodeElement.appendChild(idLabel);

        // Dragging
        nodeElement.addEventListener('mousedown', startDrag);
        // Delete
        nodeElement.addEventListener('dblclick', deleteNode);
    });
}

let activeNode = null;
let initialX, initialY;

function startDrag(e) {
    activeNode = e.target;
    initialX = e.clientX - activeNode.offsetLeft;
    initialY = e.clientY - activeNode.offsetTop;
    document.addEventListener('mousemove', drag);
    document.addEventListener('mouseup', endDrag);
}

async function drag(e) {
    if (activeNode) {
        activeNode.style.left = e.clientX - initialX + 'px';
        activeNode.style.top = e.clientY - initialY + 'px';
    }
}

async function endDrag() {
    if (activeNode) {
        document.removeEventListener('mousemove', drag);
        document.removeEventListener('mouseup', endDrag);

        const nodeId = parseInt(activeNode.dataset.id);
        const x = activeNode.offsetLeft;
        const y = activeNode.offsetTop;
        await invoke('update_node_position', { nodeId, x, y });
        loadGraph();
        activeNode = null;
    }
}

async function deleteNode(e) {
    const nodeId = parseInt(e.target.dataset.id);
    await invoke('delete_node', { node_id: nodeId });
    loadGraph();
}

// New function to clear the board
async function clearBoard() {
    try {
        await invoke('clear_graph');
        loadGraph(); // Reload the (now empty) graph
        console.log("Board cleared successfully.");
    } catch (error) {
        console.error("Error clearing the board:", error);
        alert("Error clearing the board.");
    }
}

window.addEventListener('DOMContentLoaded', () => {
    const addButton = document.querySelector('.footer button:nth-child(1)');
    const addEdgeButton = document.querySelector('#add-edge-button');
    // addEdgeButton.addEventListener('click', addEdgeToGrid);
    addButton.addEventListener('click', addNodeToGrid);
    addEdgeButton.addEventListener('click', addEdgeToGrid);

    // Add event listener for the clear board button
    const clearButton = document.querySelector('#clear-board-button'); // Assuming you have a button with this ID
    if (clearButton) {
        clearButton.addEventListener('click', clearBoard);
    }
    loadGraph();
});
