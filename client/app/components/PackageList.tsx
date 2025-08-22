import { Package } from "../types";

type Props = {
  packages: Package[];
}

const PackageList = ({ packages }: Props) => (
  <ul className="space-y-6">
    {packages.map(pkg => (
      <li
        key={pkg.name}
        className="bg-gradient-to-r from-fuchsia-700 via-purple-800 to-indigo-800 border-2 border-fuchsia-400 rounded-xl shadow-xl p-6 text-left hover:scale-[1.02] transition-transform duration-200 group"
      >
        <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-2">
          <div>
            <span className="text-2xl font-bold text-fuchsia-300 group-hover:text-fuchsia-200 transition-colors duration-200">
              {pkg.name}
            </span>
            <span className="ml-3 px-2 py-1 text-xs font-mono bg-indigo-900 text-indigo-300 rounded-lg border border-indigo-500">
              v{pkg.version}
            </span>
          </div>
          <div className="text-md text-purple-200 font-light mt-2 md:mt-0">
            {pkg.description}
          </div>
        </div>
      </li>
    ))}
  </ul>
);

export default PackageList;
