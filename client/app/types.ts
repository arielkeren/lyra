type File = {
  name: string;
  path: string;
  content: string;
};

export type Package = {
  name: string;
  description: string;
  version: string;
  files: File[];
};

export type User = {
  id: string;
  username: string;
  email: string;
};

export type OtherUser = {
  username: string;
  createdAt: string;
  packagesCreated: string[];
};

export const isString = (data: unknown): data is string =>
  typeof data === "string";

export const isObject = (data: unknown): data is object & {} =>
  typeof data === "object" && data !== null;

const isFile = (data: unknown): data is File => {
  if (!isObject(data)) return false;

  const file = data as File;
  return isString(file.name) && isString(file.path) && isString(file.content);
};

export const isPackage = (data: unknown): data is Package => {
  if (!isObject(data)) return false;

  const pkg = data as Package;
  return (
    isString(pkg.name) &&
    isString(pkg.description) &&
    isString(pkg.version) &&
    Array.isArray(pkg.files) &&
    pkg.files.every(file => isFile(file))
  );
};

export const isUser = (data: unknown): data is User => {
  if (!isObject(data)) return false;

  const user = data as User;
  return isString(user.id) && isString(user.username) && isString(user.email);
};

export const isOtherUser = (data: unknown): data is OtherUser => {
  if (!isObject(data)) return false;

  const user = data as OtherUser;
  return (
    isString(user.username) &&
    isString(user.createdAt) &&
    Array.isArray(user.packagesCreated) &&
    user.packagesCreated.every(pkg => isString(pkg))
  );
};
