type Props = {
  packages: string[];
};

const PackageNameList = ({ packages }: Props) => {
  if (packages.length === 0)
    return <p>This user doesn&apos;t have any packages</p>;

  return (
    <ul className="space-y-6">
      {packages.map(pkg => (
        <li
          key={pkg}
          className="bg-gradient-to-r from-fuchsia-700 via-purple-800 to-indigo-800 border-2 border-fuchsia-400 rounded-xl shadow-xl p-6 text-left hover:scale-[1.02] transition-transform duration-200 group"
        >
          {pkg}
        </li>
      ))}
    </ul>
  );
};

export default PackageNameList;
