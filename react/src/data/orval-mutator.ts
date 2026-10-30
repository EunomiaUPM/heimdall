import {
  customInstance as baseCustomInstance,
  type CustomInstanceOptions,
} from "@eunomia/ui";

export {
  setApiGatewayBase,
  getApiGatewayBase,
  UNAUTHORIZED_EVENT,
  type RequestConfig,
  type CustomInstanceOptions,
  type ErrorType,
  type BodyType,
} from "@eunomia/ui";

export interface OrvalRequestOptions extends Omit<CustomInstanceOptions, "headers"> {
  headers?: HeadersInit | Record<string, string | readonly string[] | undefined>;
}

export const customInstance = async <T>(
  url: string,
  options?: OrvalRequestOptions,
): Promise<T> => {
  if (!options?.headers || options.headers instanceof Headers || Array.isArray(options.headers)) {
    return baseCustomInstance<T>(url, options as CustomInstanceOptions | undefined);
  }

  const normalized: Record<string, string> = {};
  for (const [k, v] of Object.entries(options.headers)) {
    if (v !== undefined) {
      normalized[k] = Array.isArray(v) ? v.join(", ") : String(v);
    }
  }

  return baseCustomInstance<T>(url, {
    ...options,
    headers: normalized,
  });
};
