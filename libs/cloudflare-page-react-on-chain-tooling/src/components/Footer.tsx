import { FC } from 'react'


const Footer: FC = () => {
  return (
    <footer className="bg-gray-900 border-t border-t-gray-600">
      <div className="mx-auto max-w-7xl px-6 pb-8 pt-16 sm:pt-24 lg:px-8 lg:pt-32">
        <div className="xl:grid xl:grid-cols-3 xl:gap-8">
          <div className="space-y-8">
            <img className="h-9" src="https://r2-images.blockmesh.xyz/recyclo-no-bg.png" alt="" />
            <p className="text-balance text-sm/6 text-gray-300">
              Don't be a token waster, recycle faster
            </p>
            <div className="flex gap-x-6">
              <a href="https://x.com/_cycoin_" className="text-gray-400 hover:text-gray-300">
                <span className="sr-only">X</span>
                <svg className="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path
                    d="M13.6823 10.6218L20.2391 3H18.6854L12.9921 9.61788L8.44486 3H3.2002L10.0765 13.0074L3.2002 21H4.75404L10.7663 14.0113L15.5685 21H20.8131L13.6819 10.6218H13.6823ZM11.5541 13.0956L10.8574 12.0991L5.31391 4.16971H7.70053L12.1742 10.5689L12.8709 11.5655L18.6861 19.8835H16.2995L11.5541 13.096V13.0956Z" />
                </svg>
              </a>
            </div>
          </div>
        </div>
      </div>
    </footer>
  )
}

export default Footer