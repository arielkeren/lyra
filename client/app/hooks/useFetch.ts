import { useState, useEffect } from "react";

const useFetch = <T>(get: () => Promise<T | null>) => {
  const [data, setData] = useState<T | null | undefined>(undefined);

  useEffect(() => {
    const fetchPackages = async () => {
      setData(await get());
    };

    fetchPackages();
  }, [get]);

  return data;
};

export default useFetch;
