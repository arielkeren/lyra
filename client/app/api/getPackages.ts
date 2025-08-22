import { get } from "./methods";
import { isGetPackagesResponse } from "./types";

const getPackages = async () => {
  const data = await get("packages");
  return isGetPackagesResponse(data) ? data.packages : null;
};

export default getPackages;
