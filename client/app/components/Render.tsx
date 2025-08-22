type Props<T> = {
  data: T | null | undefined;
  loading: React.ReactNode;
  error: React.ReactNode;
  content: (data: T) => React.ReactNode;
};

const Render = <T,>({ data, loading, error, content }: Props<T>) => {
  if (data === undefined) return loading;
  if (data === null) return error;
  return content(data);
};

export default Render;
