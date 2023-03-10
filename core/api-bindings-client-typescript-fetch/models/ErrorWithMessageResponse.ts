/* tslint:disable */
/* eslint-disable */
/**
 * Prelude API
 * Primary REST API for Prelude
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: sophie@sophiekatz.us
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface ErrorWithMessageResponse
 */
export interface ErrorWithMessageResponse {
    /**
     * 
     * @type {string}
     * @memberof ErrorWithMessageResponse
     */
    message: string;
}

/**
 * Check if a given object implements the ErrorWithMessageResponse interface.
 */
export function instanceOfErrorWithMessageResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "message" in value;

    return isInstance;
}

export function ErrorWithMessageResponseFromJSON(json: any): ErrorWithMessageResponse {
    return ErrorWithMessageResponseFromJSONTyped(json, false);
}

export function ErrorWithMessageResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ErrorWithMessageResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'message': json['message'],
    };
}

export function ErrorWithMessageResponseToJSON(value?: ErrorWithMessageResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'message': value.message,
    };
}

