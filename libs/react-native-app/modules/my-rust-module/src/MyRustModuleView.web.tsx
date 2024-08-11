import * as React from 'react';

import { MyRustModuleViewProps } from './MyRustModule.types';

export default function MyRustModuleView(props: MyRustModuleViewProps) {
  return (
    <div>
      <span>{props.name}</span>
    </div>
  );
}
