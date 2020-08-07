import { DateTime, State, Device } from '.';
import { DocumentReference } from '@firebase/firestore-types';

export namespace Client {
  export type RequestType = 'TEST' & Device.RequestType;

  export interface Request {
    requestType: RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | State;
  }

  export type ResponseType = 'DATA' | 'DEVICES' | 'DEVICES_STATUS';

  export interface Response {
    requestType: ResponseType;
    data?: Device.ActiveDevice[] | Device.FirebaseDevice[];
  }
  export interface FirebaseUser {
    devices: {
      full_access: DocumentReference[];
    };
    permission: number;
  }
}
