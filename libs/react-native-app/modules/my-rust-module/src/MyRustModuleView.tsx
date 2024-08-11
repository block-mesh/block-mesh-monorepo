import { requireNativeViewManager } from 'expo-modules-core';
import * as React from 'react';

import { MyRustModuleViewProps } from './MyRustModule.types';

const NativeView: React.ComponentType<MyRustModuleViewProps> =
  requireNativeViewManager('MyRustModule');

export default function MyRustModuleView(props: MyRustModuleViewProps) {
  return <NativeView {...props} />;
}
