const path = require('path');
const fs = require('fs');

const input = fs
	.readFileSync(path.join(__dirname, 'data.txt'), 'utf8')
	.toString()
	.trim()
	.split('\r\n');

module.exports = input;