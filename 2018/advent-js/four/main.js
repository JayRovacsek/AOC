main();

function GuardLog (time,guard,action) {
    this.time = time;
    this.action = action;
    this.guard = guard;

    function getTime(){
        return time;
    }

    function getAction(){
        return action;
    }

    function getGuard(){
        return guard;
    }

    function setAction(action){
        this.action = action;
    }

    return {
        getTime: getTime,
        getAction: getAction,
        setAction: setAction,
        getGuard: getGuard
    }
}

function Guard(id) {
    this.id = id;
    this.totalTime = 0;
    this.asleepTime = 0;

    function getId(){
        return id;
    }

    function addTotalTime(time){
        totalTime += time;
    }

    function addAsleepTime(time){
        asleepTime += time;
    }

    return {
        getId: getId,
        addTotalTime: addTotalTime,
        addAsleepTime: addAsleepTime
    }
}

function main(){
    var guardLogs = loadGuardLogs();
    var sortedLogs = guardLogs.sort(function(a,b){return a-b});
    sortedLogs.forEach(element => {
        var time = element.getTime();
        console.log(time);
    });
}

function loadData(filename){
    var fs = require('fs');
    return fs.readFileSync(filename).toString().split("\n");
}

function parseDate(date){
    var year = date.substring(0,4);
    var month = parseInt(date.substring(5,7))-1;
    var day = parseInt(date.substring(8,10));
    var hour = parseInt(date.substring(11,13));
    var minute = date.substring(14,16);
    return new Date(year,month,day,hour,minute,0,0);
}

function loadGuardLogs(){
    var lines = loadData('input.txt');

    var guards = [];
    var guardLogs = [];
    lines.forEach(line => {
        var dt = parseDate(line.substring(1,17));
        if(line.search("#") != -1){
            var id = line.split(" ")[3].replace("#","");
            var guard = new Guard(id);
            var log = new GuardLog(dt,guard)
            if(!guards.find(x => x.getId() == id)){
                guards.push(guard);
                log.setAction("Start");
                guardLogs.push(log);
            } else {
                log.setAction("Start");
                guardLogs.push(log);
            }
        }
        else {
            if(line.search("asleep") != -1){
                var log = new GuardLog(dt)
                log.setAction("Asleep");
                guardLogs.push(log);
            }
            if(line.search("wakes") != -1){
                var log = new GuardLog(dt)
                log.setAction("Wakes");
                guardLogs.push(log);
                console.log(log.getAction());
            }
        }

    });

    console.log("Total logs: "+guardLogs.length)
    console.log("Total alseeps: "+guardLogs.sort(x => x.time).forEach(x => x.getAction()));
    console.log("Total wakes: "+guardLogs.find(x => x.action == "Wakes"));

    return guardLogs;
}