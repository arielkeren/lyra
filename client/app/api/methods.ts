export const get = async (route: string) => {
  if (!process.env.NEXT_PUBLIC_API_URL)
    throw new Error("API URL is not defined");

  try {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/${route}`);
    if (!res.ok) return null;

    return (await res.json()) as unknown;
  } catch {
    return null;
  }
};

export const post = async (route: string, body: object) => {
  if (!process.env.NEXT_PUBLIC_API_URL)
    throw new Error("API URL is not defined");

  try {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/${route}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) return null;

    return (await res.json()) as unknown;
  } catch {
    return null;
  }
};

export const put = async (route: string, token: string, body: object) => {
  if (!process.env.NEXT_PUBLIC_API_URL)
    throw new Error("API URL is not defined");

  try {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/${route}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) return null;

    return (await res.json()) as unknown;
  } catch {
    return false;
  }
};
