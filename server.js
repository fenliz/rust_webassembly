var express = require('express');
var path = require('path');
var app = express();

app.set('port', 3000);

app.use('/', express.static(path.join(__dirname, 'public')));

console.log(path.join(__dirname, 'www'));
app.listen(app.get('port'), function() {
   console.log('Listening on port ' + app.get('port'));
});