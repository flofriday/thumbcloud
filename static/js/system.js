var uptimeElement = document.getElementById("uptime");

var wsUri = 'ws:' + window.location.host + '/ws/';
console.log('Trying to connect to: ' + wsUri);
var conn = new WebSocket(wsUri);

conn.onopen = function(e) {
    console.log('Connected.');
    requestUptime();
}

conn.onmessage = function(e) {
    decode(e.data);
};

conn.onclose = function() {
    console.log('Disconnected.');
    displayErrorAndReload('Disconnected','Lost connection to the server.');
    conn = null;
};

function requestUptime(path) {
    msg = {
        "action": "requestUptime",
    };

    conn.send(JSON.stringify(msg));
}

function decode(input) {
    obj = JSON.parse(input)

    if (obj.action == 'sendUptime') {
        renderUptime(obj.uptime);

    } else if (obj.action == 'sendError') {
        console.log('Got Error from Server:' + obj.message);
        displayError('Internal Server Error', obj.message);

    } else {
        console.log('Got invalid "' + obj.action + '" as action from sever')
    }
}

function renderUptime(input) {
    if (conn == null) {
        return;
    }

    var seconds = input;
    var days = Math.floor(seconds / (3600*24));
    seconds  -= days*3600*24;
    var hrs   = Math.floor(seconds / 3600);
    seconds  -= hrs*3600;
    var mins = Math.floor(seconds / 60);
    seconds  -= mins*60;

    var output = "";
    if (days > 0) {output += days + " days, "}
    if (days > 0 || hrs > 0) {output += hrs + " hours, "}
    if (days > 0 || hrs > 0 || mins > 0) {output += mins + " minutes, "}
    output += seconds + " seconds"

    uptimeElement.innerHTML = output;
    setTimeout(function(){renderUptime(input+1)}, 1000);
}

function displayError(header, message) {
    $('#errorModalLabel').text(header);
    $('#errorModalContent').text(message);
    $('#errorModal').modal('show');
}

function displayErrorAndReload(header, message) {
    $('#errorReloadModalLabel').text(header);
    $('#errorReloadModalContent').text(message);
    $('#errorReloadModal').modal('show');
    $('#errorReloadModal').on('hide.bs.modal', function (e) {
        window.location.reload();
    })
}
