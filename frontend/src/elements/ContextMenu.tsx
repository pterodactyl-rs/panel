import { createContext, useContext, useState, useRef, useEffect } from 'react';
import { createPortal } from 'react-dom';
import { AnimatePresence, motion } from 'motion/react';
import classNames from 'classnames';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faEllipsis, IconDefinition } from '@fortawesome/free-solid-svg-icons';

const ContextMenuContext = createContext(null);

interface Item {
  icon: IconDefinition;
  label: string;
  hidden?: boolean;
  onClick: () => void;
  color: 'gray' | 'red';
}

export const ContextMenuProvider = ({ children }) => {
  const [state, setState] = useState({
    visible: false,
    x: 0,
    y: 0,
    items: [] as Item[],
  });

  const [shouldRender, setShouldRender] = useState(false);
  const menuRef = useRef(null);

  const showMenu = (x, y, items) => {
    setState({ visible: true, x, y, items });
    setShouldRender(true);
  };

  const hideMenu = () => {
    setState((prev) => ({ ...prev, visible: false }));
  };

  useEffect(() => {
    const handleClickOutside = (e) => {
      if (menuRef.current && !menuRef.current.contains(e.target)) {
        hideMenu();
      }
    };

    if (state.visible) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [state.visible]);

  return (
    <ContextMenuContext.Provider value={{ state, showMenu, hideMenu }}>
      {children}
      {shouldRender &&
        createPortal(
          <AnimatePresence
            onExitComplete={() => {
              setShouldRender(false);
            }}
          >
            {state.visible && (
              <motion.ul
                ref={menuRef}
                className={'p-2 absolute z-50 bg-gray-600 border border-gray-500 shadow-md rounded w-fit'}
                style={{ top: state.y, left: state.x }}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.5 }}
                transition={{ duration: 0.07 }}
              >
                {state.items
                  .filter((item) => !item.hidden)
                  .map((item, idx) => (
                    <li
                      key={idx}
                      className={classNames(
                        'px-4 py-2 rounded cursor-pointer',
                        item.color === 'red' ? 'hover:text-red-100 hover:bg-red-500 ' : 'hover:bg-gray-500',
                      )}
                      onClick={() => {
                        item.onClick();
                        hideMenu();
                      }}
                    >
                      <FontAwesomeIcon fixedWidth icon={item.icon} className={'mr-2'} />
                      {item.label}
                    </li>
                  ))}
              </motion.ul>
            )}
          </AnimatePresence>,
          document.body,
        )}
    </ContextMenuContext.Provider>
  );
};

const ContextMenu = ({ items = [], children }: { items: Item[]; children: any }) => {
  const { showMenu, hideMenu } = useContext(ContextMenuContext);

  const openMenu = (x, y) => {
    showMenu(
      x,
      y,
      items.filter((item) => item), // Filter null values
    );
  };

  return children({ openMenu, hideMenu });
};

ContextMenu.Toggle = ({ openMenu }: { openMenu: (x: number, y: number) => void }) => {
  return (
    <td
      className={'relative cursor-pointer w-10 text-center'}
      onClick={(e) => {
        e.stopPropagation();
        const rect = e.currentTarget.getBoundingClientRect();
        openMenu(rect.left, rect.bottom);
      }}
    >
      <FontAwesomeIcon icon={faEllipsis} />
    </td>
  );
};

export default ContextMenu;
