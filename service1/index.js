const http = require('http');
const arg = process.argv[2];

const port = arg ? parseInt(arg) : 3001;

const server = http.createServer((req, res) => {
    console.log('Service2: ' + req.url + ' - ' + req.method);
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
        message: 'Service1 is running on port ' + port
    }));
});

server.listen(port, () => {
    console.log(`Service1 is running on http://localhost:${port}`);
});