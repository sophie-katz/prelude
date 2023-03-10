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
import type { ConfigurationTypeResponse } from './ConfigurationTypeResponse';
import {
    ConfigurationTypeResponseFromJSON,
    ConfigurationTypeResponseFromJSONTyped,
    ConfigurationTypeResponseToJSON,
} from './ConfigurationTypeResponse';

/**
 * A single configuration key
 * @export
 * @interface ConfigurationKeyResponse
 */
export interface ConfigurationKeyResponse {
    /**
     * A unique id for the object
     * @type {number}
     * @memberof ConfigurationKeyResponse
     */
    id: number;
    /**
     * A human-readable name for the object
     * @type {string}
     * @memberof ConfigurationKeyResponse
     */
    name: string;
    /**
     * A human-readable description of the object
     * @type {string}
     * @memberof ConfigurationKeyResponse
     */
    description: string;
    /**
     * 
     * @type {ConfigurationTypeResponse}
     * @memberof ConfigurationKeyResponse
     */
    type: ConfigurationTypeResponse;
    /**
     * Whether or not the key is optional
     * @type {boolean}
     * @memberof ConfigurationKeyResponse
     */
    optional: boolean;
    /**
     * Whether or not the key allows multiple values in an ordered list
     * @type {boolean}
     * @memberof ConfigurationKeyResponse
     */
    allowsMultiple: boolean;
    /**
     * Whether or not the key allows users to override the configuration value in their personal profiles
     * @type {boolean}
     * @memberof ConfigurationKeyResponse
     */
    allowsUserOverride: boolean;
}

/**
 * Check if a given object implements the ConfigurationKeyResponse interface.
 */
export function instanceOfConfigurationKeyResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "id" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "description" in value;
    isInstance = isInstance && "type" in value;
    isInstance = isInstance && "optional" in value;
    isInstance = isInstance && "allowsMultiple" in value;
    isInstance = isInstance && "allowsUserOverride" in value;

    return isInstance;
}

export function ConfigurationKeyResponseFromJSON(json: any): ConfigurationKeyResponse {
    return ConfigurationKeyResponseFromJSONTyped(json, false);
}

export function ConfigurationKeyResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ConfigurationKeyResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': json['id'],
        'name': json['name'],
        'description': json['description'],
        'type': ConfigurationTypeResponseFromJSON(json['type']),
        'optional': json['optional'],
        'allowsMultiple': json['allowsMultiple'],
        'allowsUserOverride': json['allowsUserOverride'],
    };
}

export function ConfigurationKeyResponseToJSON(value?: ConfigurationKeyResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'name': value.name,
        'description': value.description,
        'type': ConfigurationTypeResponseToJSON(value.type),
        'optional': value.optional,
        'allowsMultiple': value.allowsMultiple,
        'allowsUserOverride': value.allowsUserOverride,
    };
}

