import redis from 'redis';
import { Device } from '@gbaranski/types';
import { SubChannel } from '@/types';

export let activeDevices: Device.ActiveDevice[] = [];

export const redisSubscriber = redis.createClient('redis://redis:6379');
redisSubscriber.on('connect', () =>
  console.log('Initialized redis subscriber'),
);
redisSubscriber.subscribe('device_data');
redisSubscriber.subscribe('device_disconnect');
redisSubscriber.on('message', (channel, message) => {
  const targetChannel = channels.find((_channel) => _channel.name === channel);
  if (!targetChannel) throw new Error('Channel unrecognized');
  targetChannel.handle(message);
});

const handleNewDeviceData = (message: string) => {
  const activeDevice = JSON.parse(message) as Device.ActiveDevice;

  activeDevices = activeDevices
    .filter((device) => activeDevice.uid !== device.uid)
    .concat(activeDevice);
};

const handleDeviceDisconnect = (message: string) => {
  const activeDevice = JSON.parse(message) as Device.ActiveDevice;
  activeDevices = activeDevices.filter(
    (device) => device.uid !== activeDevice.uid,
  );
};

const channels: SubChannel[] = [
  {
    name: 'device_data',
    handle: handleNewDeviceData,
  },
  {
    name: 'device_disconnect',
    handle: handleDeviceDisconnect,
  },
];
