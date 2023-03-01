/* tslint:disable */
/* eslint-disable */
/**
 * Portobello API
 * Primary REST API for Portobello
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
 * A value for a configuration key.
 * 
 * **NOTE:** This is essentially a union of the property types.
 * @export
 * @interface ConfigurationValueResponse
 */
export interface ConfigurationValueResponse {
    /**
     * The value as a boolean
     * @type {boolean}
     * @memberof ConfigurationValueResponse
     */
    asBoolean?: boolean;
    /**
     * The value as an integer
     * @type {number}
     * @memberof ConfigurationValueResponse
     */
    asInteger?: number;
    /**
     * The value as a floating-point number
     * @type {number}
     * @memberof ConfigurationValueResponse
     */
    asFloat?: number;
    /**
     * The value as a string
     * @type {string}
     * @memberof ConfigurationValueResponse
     */
    asString?: string;
}

/**
 * Check if a given object implements the ConfigurationValueResponse interface.
 */
export function instanceOfConfigurationValueResponse(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function ConfigurationValueResponseFromJSON(json: any): ConfigurationValueResponse {
    return ConfigurationValueResponseFromJSONTyped(json, false);
}

export function ConfigurationValueResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ConfigurationValueResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'asBoolean': !exists(json, 'asBoolean') ? undefined : json['asBoolean'],
        'asInteger': !exists(json, 'asInteger') ? undefined : json['asInteger'],
        'asFloat': !exists(json, 'asFloat') ? undefined : json['asFloat'],
        'asString': !exists(json, 'asString') ? undefined : json['asString'],
    };
}

export function ConfigurationValueResponseToJSON(value?: ConfigurationValueResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'asBoolean': value.asBoolean,
        'asInteger': value.asInteger,
        'asFloat': value.asFloat,
        'asString': value.asString,
    };
}

