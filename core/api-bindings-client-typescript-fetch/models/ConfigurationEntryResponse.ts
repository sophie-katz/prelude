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
import type { ConfigurationEntryItemSetResponse } from './ConfigurationEntryItemSetResponse';
import {
    ConfigurationEntryItemSetResponseFromJSON,
    ConfigurationEntryItemSetResponseFromJSONTyped,
    ConfigurationEntryItemSetResponseToJSON,
} from './ConfigurationEntryItemSetResponse';
import type { ConfigurationEntryUserResponse } from './ConfigurationEntryUserResponse';
import {
    ConfigurationEntryUserResponseFromJSON,
    ConfigurationEntryUserResponseFromJSONTyped,
    ConfigurationEntryUserResponseToJSON,
} from './ConfigurationEntryUserResponse';
import type { ConfigurationKeyResponse } from './ConfigurationKeyResponse';
import {
    ConfigurationKeyResponseFromJSON,
    ConfigurationKeyResponseFromJSONTyped,
    ConfigurationKeyResponseToJSON,
} from './ConfigurationKeyResponse';

/**
 * An entry representing the stored value of a configuration key
 * @export
 * @interface ConfigurationEntryResponse
 */
export interface ConfigurationEntryResponse {
    /**
     * 
     * @type {ConfigurationKeyResponse}
     * @memberof ConfigurationEntryResponse
     */
    key: ConfigurationKeyResponse;
    /**
     * 
     * @type {ConfigurationEntryItemSetResponse}
     * @memberof ConfigurationEntryResponse
     */
    itemsGlobal: ConfigurationEntryItemSetResponse;
    /**
     * 
     * @type {ConfigurationEntryUserResponse}
     * @memberof ConfigurationEntryResponse
     */
    user: ConfigurationEntryUserResponse | null;
}

/**
 * Check if a given object implements the ConfigurationEntryResponse interface.
 */
export function instanceOfConfigurationEntryResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "key" in value;
    isInstance = isInstance && "itemsGlobal" in value;
    isInstance = isInstance && "user" in value;

    return isInstance;
}

export function ConfigurationEntryResponseFromJSON(json: any): ConfigurationEntryResponse {
    return ConfigurationEntryResponseFromJSONTyped(json, false);
}

export function ConfigurationEntryResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ConfigurationEntryResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'key': ConfigurationKeyResponseFromJSON(json['key']),
        'itemsGlobal': ConfigurationEntryItemSetResponseFromJSON(json['itemsGlobal']),
        'user': ConfigurationEntryUserResponseFromJSON(json['user']),
    };
}

export function ConfigurationEntryResponseToJSON(value?: ConfigurationEntryResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'key': ConfigurationKeyResponseToJSON(value.key),
        'itemsGlobal': ConfigurationEntryItemSetResponseToJSON(value.itemsGlobal),
        'user': ConfigurationEntryUserResponseToJSON(value.user),
    };
}

