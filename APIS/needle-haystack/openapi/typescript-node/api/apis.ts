export * from './recordApi';
import { RecordApi } from './recordApi';
export * from './satalitteApi';
import { SatalitteApi } from './satalitteApi';
export * from './satelliteImageApi';
import { SatelliteImageApi } from './satelliteImageApi';
export * from './userApi';
import { UserApi } from './userApi';
import * as http from 'http';

export class HttpError extends Error {
    constructor (public response: http.IncomingMessage, public body: any, public statusCode?: number) {
        super('HTTP request failed');
        this.name = 'HttpError';
    }
}

export { RequestFile } from '../model/models';

export const APIS = [RecordApi, SatalitteApi, SatelliteImageApi, UserApi];
