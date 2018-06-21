

function retnum(str) {
    //var num = str.replace(/[^0-9]/g, '');
    return parseInt(str,10);
}

function drawChart(ctx, v0, v1, v2, v3, v4)
{
var v0n = retnum(v0);
var v1n = retnum(v1);
var v2n = retnum(v2);
var v3n = retnum(v3);
var v4n = retnum(v4);

    var myChart = new Chart(ctx,  {
        type: 'pie',
        data: {
            labels: ["Forward", "Backward", "Reverse", "Inward", "Twisting"],
            datasets: [{
                label: '# of Votes',
                data: [v0n, v1n, v2n, v3n, v4n ],
                backgroundColor: [
                    'rgba(255, 99, 132, 0.2)',
                    'rgba(54, 162, 235, 0.2)',
                    'rgba(255, 206, 86, 0.2)',
                    'rgba(75, 192, 192, 0.2)',
                    'rgba(153, 102, 255, 0.2)'
                ],
                borderColor: [
                    'rgba(255,99,132,1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(255, 206, 86, 1)',
                    'rgba(75, 192, 192, 1)',
                    'rgba(153, 102, 255, 1)'
                ],
                borderWidth: 1
            }]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }],
                "xAxes":[]
            }
        }
    });
}


function drawChart3(ctx, v0, v1, v2)
{
var v0n = retnum(v0);
var v1n = retnum(v1);
var v2n = retnum(v2);


    var myChart = new Chart(ctx,  {
        type: 'pie',
        data: {
            labels: ["1m", "3m", "Plattform"],
            datasets: [{
                label: '# of Votes',
                data: [v0n, v1n, v2n ],
                backgroundColor: [
                    'rgba(255, 99, 132, 0.2)',
                    'rgba(54, 162, 235, 0.2)',
                    'rgba(255, 206, 86, 0.2)'
                ],
                borderColor: [
                    'rgba(255,99,132,1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(255, 206, 86, 1)'
                ],
                borderWidth: 1
            }]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }],
                "xAxes":[]
            }
        }
    });
}


function drawBarChart(ctx)
{

    var myChart = new Chart(ctx, {
        type: 'pie',
        data: {
            labels: ["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
            datasets: [{
                label: '# of Votes',
                data: [12, 19, 3, 5, 2, 3],
                backgroundColor: [
                    'rgba(255, 99, 132, 0.2)',
                    'rgba(54, 162, 235, 0.2)',
                    'rgba(255, 206, 86, 0.2)',
                    'rgba(75, 192, 192, 0.2)',
                    'rgba(153, 102, 255, 0.2)',
                    'rgba(255, 159, 64, 0.2)'
                ],
                borderColor: [
                    'rgba(255,99,132,1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(255, 206, 86, 1)',
                    'rgba(75, 192, 192, 1)',
                    'rgba(153, 102, 255, 1)',
                    'rgba(255, 159, 64, 1)'
                ],
                borderWidth: 1
            }]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }],
                "xAxes":[]
            }
        }
    });
}


function drawDiveGraph(ctx, ds, tt)
{
    var myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: tt,
            datasets: [{
                label: '# of times',
                data: ds,
                backgroundColor: 'rgba(255, 99, 132, 0.2)'
                ,
                borderColor:
                    'rgba(255, 159, 64, 1)'
                ,
                borderWidth: 1
            }]
        },
        options: {
            title: {
              display: true,
              text: 'Amount of times dive is made per training date.'
            }
          }
    });
}



function drawCompetitionDiveGraph(ctx, ds, tt)
{
    var myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: tt,
            datasets: [{
                label: 'score',
                data: ds,
                backgroundColor: 'rgba(255, 99, 132, 0.2)'
                ,
                borderColor:
                    'rgba(255, 159, 64, 1)'
                ,
                borderWidth: 1
            }]
        },
        options: {
            title: {
              display: true,
              text: 'Total score on dive per competition date'
            }
          }
    });
}

