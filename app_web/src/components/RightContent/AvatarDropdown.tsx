import React, { useCallback } from 'react';
import { LogoutOutlined, SettingOutlined, UserOutlined } from '@ant-design/icons';
import { Avatar, Menu, Spin } from 'antd';
import { history, useModel } from 'umi';
import { firebaseAuth } from '@/services/firebase';
import { AvatarProps } from 'antd/lib/avatar';
import HeaderDropdown from '../HeaderDropdown';
import styles from './index.less';

export interface GlobalHeaderRightProps {
  menu?: boolean;
}

/**
 * 退出登录，并且将当前的 url 保存
 */
const signOut = async () => {
  await firebaseAuth.signOut();
  history.push('/user/login');
};

const AvatarDropdown: React.FC<GlobalHeaderRightProps> = ({ menu }) => {
  const { initialState, setInitialState } = useModel('@@initialState');

  const onMenuClick = useCallback(
    (event: {
      key: React.Key;
      keyPath: React.Key[];
      item: React.ReactInstance;
      domEvent: React.MouseEvent<HTMLElement>;
    }) => {
      const { key } = event;
      if (key === 'logout') {
        setInitialState({
          ...initialState,
          currentUser: undefined,
          firebaseUser: undefined,
          mqtt: undefined,
        });
        signOut();
        return;
      }
      history.push(`/account/${key}`);
    },
    [],
  );

  const loading = (
    <span className={`${styles.action} ${styles.account}`}>
      <Spin
        size="small"
        style={{
          marginLeft: 8,
          marginRight: 8,
        }}
      />
    </span>
  );

  if (!initialState) {
    console.log('Initialstate is not defined');
    return loading;
  }

  const { currentUser, firebaseUser } = initialState;

  if (!currentUser || !firebaseUser) return loading;

  const menuHeaderDropdown = (
    <Menu className={styles.menu} selectedKeys={[]} onClick={onMenuClick}>
      {menu && (
        <Menu.Item key="center">
          <UserOutlined />
          个人中心
        </Menu.Item>
      )}
      {menu && (
        <Menu.Item key="settings">
          <SettingOutlined />
          个人设置
        </Menu.Item>
      )}
      {menu && <Menu.Divider />}

      <Menu.Item key="logout">
        <LogoutOutlined />
        Log out
      </Menu.Item>
    </Menu>
  );

  const CustomAvatar = (props: AvatarProps) => (
    <Avatar size="small" className={styles.avatar} alt="avatar" {...props} />
  );

  return (
    <HeaderDropdown overlay={menuHeaderDropdown}>
      <span className={`${styles.action} ${styles.account}`}>
        {currentUser.photoURL ? (
          <CustomAvatar src={currentUser.photoURL} />
        ) : (
          <CustomAvatar icon={<UserOutlined style={{ color: 'black' }} />} />
        )}
        <span className={`${styles.name} anticon`}>
          {currentUser.displayName || firebaseUser.username}
        </span>
      </span>
    </HeaderDropdown>
  );
};

export default AvatarDropdown;
