require("./tracing");

import express from "express";
import { json } from "body-parser";
import { MeterProvider } from "@opentelemetry/metrics";
import {
  ExporterConfig,
  PrometheusExporter,
} from "@opentelemetry/exporter-prometheus";

const app = express();
const PORT = 8000;

const firstName = [
  "Baby Oil",
  "Bad News",
  "Big Burps",
  "Bill 'Beenie-Weenie'",
  "Bob 'Stinkbug'",
  "Bowel Noises",
  "Boxelder",
  "Bud 'Lite'",
  "Butterbean",
  "Buttermilk",
  "Buttocks",
  "Chad",
  "Chesterfield",
  "Chewy",
  "Chigger",
  "Cinnabuns",
  "Cleet",
  "Cornbread",
  "Crab Meat",
  "Crapps",
  "Dark Skies",
  "Dennis Clawhammer",
  "Dicman",
  "Elphonso",
  "Fancypants",
  "Figgs",
  "Foncy",
  "Gootsy",
  "Greasy Jim",
  "Huckleberry",
  "Huggy",
  "Ignatious",
  "Jimbo",
  "Joe 'Pottin Soil'",
  "Johnny",
  "Lemongrass",
  "Lil Debil",
  "Longbranch",
  '"Lunch Money"',
  "Mergatroid",
  '"Mr Peabody"',
  "Oil-Can",
  "Oinks",
  "Old Scratch",
  "Ovaltine",
  "Pennywhistle",
  "Pitchfork Ben",
  "Potato Bug",
  "Pushmeet",
  "Rock Candy",
  "Schlomo",
  "Scratchensniff",
  "Scut",
  "Sid 'The Squirts'",
  "Skidmark",
  "Slaps",
  "Snakes",
  "Snoobs",
  "Snorki",
  "Soupcan Sam",
  "Spitzitout",
  "Squids",
  "Stinky",
  "Storyboard",
  "Sweet Tea",
  "TeeTee",
  "Wheezy Joe",
  "Winston 'Jazz Hands'",
  "Worms",
];

const lastName = [
  "Appleyard",
  "Bigmeat",
  "Bloominshine",
  "Boogerbottom",
  "Breedslovetrout",
  "Butterbaugh",
  "Clovenhoof",
  "Clutterbuck",
  "Cocktoasten",
  "Endicott",
  "Fewhairs",
  "Gooberdapple",
  "Goodensmith",
  "Goodpasture",
  "Guster",
  "Henderson",
  "Hooperbag",
  "Hoosenater",
  "Hootkins",
  "Jefferson",
  "Jenkins",
  "Jingley-Schmidt",
  "Johnson",
  "Kingfish",
  "Listenbee",
  "M'Bembo",
  "McFadden",
  "Moonshine",
  "Nettles",
  "Noseworthy",
  "Olivetti",
  "Outerbridge",
  "Overpeck",
  "Overturf",
  "Oxhandler",
  "Pealike",
  "Pennywhistle",
  "Peterson",
  "Pieplow",
  "Pinkerton",
  "Porkins",
  "Putney",
  "Quakenbush",
  "Rainwater",
  "Rosenthal",
  "Rubbins",
  "Sackrider",
  "Snuggleshine",
  "Splern",
  "Stevens",
  "Stroganoff",
  "Sugar-Gold",
  "Swackhamer",
  "Tippins",
  "Turnipseed",
  "Vinaigrette",
  "Walkingstick",
  "Wallbanger",
  "Weewax",
  "Weiners",
  "Whipkey",
  "Wigglesworth",
  "Wimplesnatch",
  "Winterkorn",
  "Woolysocks",
];

const meter_host = process.env.METER_HOST
  ? process.env.METER_HOST
  : "localhost";
const exporter = new PrometheusExporter({
  endpoint: meter_host,
  port: 9090,
});
const meter = new MeterProvider({
  exporter,
  interval: 1000,
}).getMeter("SidekickNameServer");

const requestCount = meter.createCounter("request_count", {
  description: "Counts total number of requests",
});
// const errorCount = meter.createCounter("error_count", {
//   description: "Counts total number of errors",
// });
const responseLatency = meter.createUpDownCounter("response_latency", {
  description: "Records latency of response",
});

app.use(json());
app.get("/name", (req, res) => {
  const requestReceived = new Date().getTime();
  requestCount.add(1);
  const selectedFirstName =
    firstName[Math.floor(Math.random() * firstName.length)];
  const selectedLastName =
    lastName[Math.floor(Math.random() * lastName.length)];
  const measuredLatency = new Date().getTime() - requestReceived;
  responseLatency.add(measuredLatency);
  res.send({ name: `${selectedFirstName} ${selectedLastName}` });
});
app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
