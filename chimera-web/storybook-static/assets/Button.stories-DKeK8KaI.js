import{j as e}from"./jsx-runtime-DF2Pcvd1.js";import"./index-B2-qRKKC.js";import"./_commonjsHelpers-Cpj98o6Y.js";function a({variant:H="primary",size:F="md",loading:r=!1,icon:p,children:G,disabled:J,className:K="",...Q}){const U=["chimera-button",`chimera-button--${H}`,`chimera-button--${F}`,r&&"chimera-button--loading",K].filter(Boolean).join(" ");return e.jsxs("button",{className:U,disabled:J||r,"aria-busy":r,...Q,children:[r&&e.jsx("span",{className:"chimera-button__spinner","aria-label":"Loading",children:"⏳"}),!r&&p&&e.jsx("span",{className:"chimera-button__icon",children:p}),e.jsx("span",{className:"chimera-button__text",children:G})]})}a.__docgenInfo={description:`Button component with multiple variants and sizes.
Supports loading state, icons, and full accessibility.

@example
\`\`\`tsx
<Button variant="primary" size="md" onClick={handleClick}>
  Click Me
</Button>
\`\`\``,methods:[],displayName:"Button",props:{variant:{required:!1,tsType:{name:"union",raw:"'primary' | 'secondary' | 'danger'",elements:[{name:"literal",value:"'primary'"},{name:"literal",value:"'secondary'"},{name:"literal",value:"'danger'"}]},description:`Button visual variant
@default 'primary'`,defaultValue:{value:"'primary'",computed:!1}},size:{required:!1,tsType:{name:"union",raw:"'sm' | 'md' | 'lg'",elements:[{name:"literal",value:"'sm'"},{name:"literal",value:"'md'"},{name:"literal",value:"'lg'"}]},description:`Button size
@default 'md'`,defaultValue:{value:"'md'",computed:!1}},loading:{required:!1,tsType:{name:"boolean"},description:`Shows loading spinner and disables button
@default false`,defaultValue:{value:"false",computed:!1}},icon:{required:!1,tsType:{name:"ReactNode"},description:"Optional icon to display before text"},children:{required:!0,tsType:{name:"ReactNode"},description:"Button content"},className:{defaultValue:{value:"''",computed:!1},required:!1}},composes:["ButtonHTMLAttributes"]};const ee={title:"Components/Button",component:a,parameters:{layout:"centered"},tags:["autodocs"],argTypes:{variant:{control:"select",options:["primary","secondary","danger"],description:"Button visual variant"},size:{control:"select",options:["sm","md","lg"],description:"Button size"},loading:{control:"boolean",description:"Shows loading spinner"},disabled:{control:"boolean",description:"Disables the button"}}},n={args:{variant:"primary",size:"md",children:"Primary Button"}},s={args:{variant:"secondary",size:"md",children:"Secondary Button"}},t={args:{variant:"danger",size:"md",children:"Danger Button"}},i={args:{size:"sm",children:"Small Button"}},o={args:{size:"md",children:"Medium Button"}},d={args:{size:"lg",children:"Large Button"}},l={args:{loading:!0,children:"Loading..."}},c={args:{disabled:!0,children:"Disabled Button"}},m={args:{icon:"🔥",children:"With Icon"}},u={render:()=>e.jsxs("div",{style:{display:"flex",gap:"10px",flexDirection:"column"},children:[e.jsxs("div",{style:{display:"flex",gap:"10px"},children:[e.jsx(a,{variant:"primary",children:"Primary"}),e.jsx(a,{variant:"secondary",children:"Secondary"}),e.jsx(a,{variant:"danger",children:"Danger"})]}),e.jsxs("div",{style:{display:"flex",gap:"10px"},children:[e.jsx(a,{variant:"primary",size:"sm",children:"Small"}),e.jsx(a,{variant:"primary",size:"md",children:"Medium"}),e.jsx(a,{variant:"primary",size:"lg",children:"Large"})]}),e.jsxs("div",{style:{display:"flex",gap:"10px"},children:[e.jsx(a,{loading:!0,children:"Loading"}),e.jsx(a,{disabled:!0,children:"Disabled"}),e.jsx(a,{icon:"🔥",children:"With Icon"})]})]})};var g,y,h;n.parameters={...n.parameters,docs:{...(g=n.parameters)==null?void 0:g.docs,source:{originalSource:`{
  args: {
    variant: 'primary',
    size: 'md',
    children: 'Primary Button'
  }
}`,...(h=(y=n.parameters)==null?void 0:y.docs)==null?void 0:h.source}}};var v,B,x;s.parameters={...s.parameters,docs:{...(v=s.parameters)==null?void 0:v.docs,source:{originalSource:`{
  args: {
    variant: 'secondary',
    size: 'md',
    children: 'Secondary Button'
  }
}`,...(x=(B=s.parameters)==null?void 0:B.docs)==null?void 0:x.source}}};var f,b,S;t.parameters={...t.parameters,docs:{...(f=t.parameters)==null?void 0:f.docs,source:{originalSource:`{
  args: {
    variant: 'danger',
    size: 'md',
    children: 'Danger Button'
  }
}`,...(S=(b=t.parameters)==null?void 0:b.docs)==null?void 0:S.source}}};var z,j,D;i.parameters={...i.parameters,docs:{...(z=i.parameters)==null?void 0:z.docs,source:{originalSource:`{
  args: {
    size: 'sm',
    children: 'Small Button'
  }
}`,...(D=(j=i.parameters)==null?void 0:j.docs)==null?void 0:D.source}}};var L,_,M;o.parameters={...o.parameters,docs:{...(L=o.parameters)==null?void 0:L.docs,source:{originalSource:`{
  args: {
    size: 'md',
    children: 'Medium Button'
  }
}`,...(M=(_=o.parameters)==null?void 0:_.docs)==null?void 0:M.source}}};var N,I,T;d.parameters={...d.parameters,docs:{...(N=d.parameters)==null?void 0:N.docs,source:{originalSource:`{
  args: {
    size: 'lg',
    children: 'Large Button'
  }
}`,...(T=(I=d.parameters)==null?void 0:I.docs)==null?void 0:T.source}}};var q,P,V;l.parameters={...l.parameters,docs:{...(q=l.parameters)==null?void 0:q.docs,source:{originalSource:`{
  args: {
    loading: true,
    children: 'Loading...'
  }
}`,...(V=(P=l.parameters)==null?void 0:P.docs)==null?void 0:V.source}}};var W,w,C;c.parameters={...c.parameters,docs:{...(W=c.parameters)==null?void 0:W.docs,source:{originalSource:`{
  args: {
    disabled: true,
    children: 'Disabled Button'
  }
}`,...(C=(w=c.parameters)==null?void 0:w.docs)==null?void 0:C.source}}};var k,A,R;m.parameters={...m.parameters,docs:{...(k=m.parameters)==null?void 0:k.docs,source:{originalSource:`{
  args: {
    icon: '🔥',
    children: 'With Icon'
  }
}`,...(R=(A=m.parameters)==null?void 0:A.docs)==null?void 0:R.source}}};var E,O,$;u.parameters={...u.parameters,docs:{...(E=u.parameters)==null?void 0:E.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    gap: '10px',
    flexDirection: 'column'
  }}>
      <div style={{
      display: 'flex',
      gap: '10px'
    }}>
        <Button variant="primary">Primary</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="danger">Danger</Button>
      </div>
      <div style={{
      display: 'flex',
      gap: '10px'
    }}>
        <Button variant="primary" size="sm">Small</Button>
        <Button variant="primary" size="md">Medium</Button>
        <Button variant="primary" size="lg">Large</Button>
      </div>
      <div style={{
      display: 'flex',
      gap: '10px'
    }}>
        <Button loading>Loading</Button>
        <Button disabled>Disabled</Button>
        <Button icon="🔥">With Icon</Button>
      </div>
    </div>
}`,...($=(O=u.parameters)==null?void 0:O.docs)==null?void 0:$.source}}};const ae=["Primary","Secondary","Danger","Small","Medium","Large","Loading","Disabled","WithIcon","AllVariants"];export{u as AllVariants,t as Danger,c as Disabled,d as Large,l as Loading,o as Medium,n as Primary,s as Secondary,i as Small,m as WithIcon,ae as __namedExportsOrder,ee as default};
