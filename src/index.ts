/* eslint-disable no-console */
import express from 'express';
import './firebase';
import Alarmclock from './alarmclock';
import Watermixer from './watermixer';
import { AlarmRequestType, WaterRequestType } from './types';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
// export GOOGLE_APPLICATION_CREDENTIALS="/Users/gbaranski/code/firebase/firebase.json"
const httpPort = 8000;

const app = express();
const alarmClock = new Alarmclock();
const waterMixer = new Watermixer();
app.use(express.json()); // for parsing application/json

// app.post('/getAlarmClock', (req, res) => {
//   console.log(req.body);
//   res.json(req.body);
// });

setInterval(async () => {
  // remove async
  alarmClock.fetchEspDataInterval();
}, 1000);

setInterval(async () => {
  // remove async
  waterMixer.fetchEspDataInterval();
}, 1000);

app.post('/api/alarmclock/getData', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_DATA);
});

app.post('/api/alarmclock/testSiren', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.TEST_ALARM);
});

app.post('/api/alarmclock/setTime', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SET_TIME);
});

app.post('/api/alarmclock/switchState', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SWITCH_STATE);
});

app.post('/api/watermixer/start', (req, res) => {
  waterMixer.handleRequest(req, res, WaterRequestType.START_MIXING);
});

app.post('/api/watermixer/getData', (req, res) => {
  waterMixer.handleRequest(req, res, WaterRequestType.GET_DATA);
});

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
