import Link from "next/link";
import useModal from "../hooks/useModal";
import useUser from "../hooks/useUser";
import LoginModal from "./LoginModal";
import RegisterModal from "./RegisterModal";
import Render from "./Render";
import SettingsModal from "./SettingsModal";

const Header = () => {
  const [isRegisterOpen, openRegister, closeRegister] = useModal();
  const [isLoginOpen, openLogin, closeLogin] = useModal();
  const [isSettingsOpen, openSettings, closeSettings] = useModal();
  const { user } = useUser();

  return (
    <>
      <RegisterModal isOpen={isRegisterOpen} onClose={closeRegister} />
      <LoginModal isOpen={isLoginOpen} onClose={closeLogin} />
      <SettingsModal isOpen={isSettingsOpen} onClose={closeSettings} />

      <header>
        <div className="max-w-7xl mx-auto px-6 py-4">
          <div className="flex justify-between items-center">
            <Link href="/" className="group">
              <h1 className="text-4xl font-extrabold tracking-tight bg-gradient-to-r from-fuchsia-300 via-pink-300 to-purple-300 bg-clip-text text-transparent drop-shadow-lg group-hover:scale-105 transition-transform duration-200">
                Lyra
              </h1>
            </Link>

            <nav className="flex items-center gap-6 text-white">
              <Link
                href="/packages"
                className="text-lg font-medium hover:text-fuchsia-300 transition-colors duration-200 hover:drop-shadow-lg"
              >
                📦 Packages
              </Link>
              <Link
                href="/docs"
                className="text-lg font-medium hover:text-fuchsia-300 transition-colors duration-200 hover:drop-shadow-lg"
              >
                📚 Docs
              </Link>

              <Render
                data={user}
                loading={
                  <div className="w-24 h-10 bg-gradient-to-r from-indigo-500/20 to-fuchsia-500/20 rounded-xl animate-pulse border border-fuchsia-400/30"></div>
                }
                error={
                  <div className="flex gap-3">
                    <button
                      onClick={openRegister}
                      className="cursor-pointer bg-gradient-to-r from-indigo-500 via-purple-500 to-fuchsia-500 hover:from-indigo-600 hover:to-fuchsia-600 text-white font-semibold py-2.5 px-5 rounded-xl shadow-lg border border-indigo-300/40 hover:border-indigo-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
                    >
                      Register
                    </button>
                    <button
                      onClick={openLogin}
                      className="cursor-pointer bg-gradient-to-r from-fuchsia-500 via-pink-500 to-purple-500 hover:from-fuchsia-600 hover:to-purple-600 text-white font-semibold py-2.5 px-5 rounded-xl shadow-lg border border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
                    >
                      Log In
                    </button>
                  </div>
                }
                content={user => (
                  <div className="flex items-center gap-4">
                    <Link
                      href={`/user/${user.id}`}
                      className="group flex items-center gap-3 bg-gradient-to-r hover:from-fuchsia-500 hover:via-pink-500 hover:to-purple-500 text-white font-semibold py-2.5 px-5 rounded-xl shadow-lg border-2 border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
                    >
                      <div className="w-8 h-8 bg-white/20 rounded-full flex items-center justify-center text-sm font-bold">
                        {user.username.charAt(0).toUpperCase()}
                      </div>
                      <span className="group-hover:text-fuchsia-100 transition-colors duration-200">
                        {user.username}
                      </span>
                    </Link>
                    <button
                      onClick={openSettings}
                      className="p-2.5 cursor-pointer bg-gradient-to-r hover:from-fuchsia-500 hover:via-pink-500 hover:to-purple-500 text-white font-semibold rounded-xl shadow-lg border-2 border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
                    >
                      <svg
                        className="w-5 h-5 text-white"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                        />
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        />
                      </svg>
                    </button>
                  </div>
                )}
              />
            </nav>
          </div>
        </div>
      </header>
    </>
  );
};

export default Header;
