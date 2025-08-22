import { isObject, isPackage, Package } from "../types";

type AuthResponse = {
  token: string;
};

type GetPackagesResponse = {
  packages: Package[];
};

export const isGetPackagesResponse = (
  data: unknown
): data is GetPackagesResponse => {
  if (!isObject(data)) return false;

  const res = data as GetPackagesResponse;
  return (
    Array.isArray(res.packages) && res.packages.every(pkg => isPackage(pkg))
  );
};

export const isAuthResponse = (data: unknown): data is AuthResponse => {
  if (!isObject(data)) return false;

  const res = data as AuthResponse;
  return typeof res.token === "string";
};
