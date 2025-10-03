import{j as e}from"./jsx-runtime-DF2Pcvd1.js";import"./index-B2-qRKKC.js";import"./_commonjsHelpers-Cpj98o6Y.js";function s({status:G="success",children:H,icon:p,className:J=""}){const K=["chimera-badge",`chimera-badge--${G}`,J].filter(Boolean).join(" ");return e.jsxs("span",{className:K,role:"status",children:[p&&e.jsx("span",{className:"chimera-badge__icon","aria-hidden":"true",children:p}),e.jsx("span",{className:"chimera-badge__text",children:H})]})}s.__docgenInfo={description:`Badge component with 3 status variants.
Small, inline display for status indicators.

@example
\`\`\`tsx
<Badge status="success">Active</Badge>
<Badge status="warning" icon="⚠️">Warning</Badge>
<Badge status="error">Failed</Badge>
\`\`\``,methods:[],displayName:"Badge",props:{status:{required:!1,tsType:{name:"union",raw:"'success' | 'warning' | 'error'",elements:[{name:"literal",value:"'success'"},{name:"literal",value:"'warning'"},{name:"literal",value:"'error'"}]},description:`Badge status variant
@default 'success'`,defaultValue:{value:"'success'",computed:!1}},children:{required:!0,tsType:{name:"ReactNode"},description:"Badge content (text)"},icon:{required:!1,tsType:{name:"ReactNode"},description:"Optional icon to display"},className:{required:!1,tsType:{name:"string"},description:"Optional custom class name",defaultValue:{value:"''",computed:!1}}}};const U={title:"Components/Badge",component:s,parameters:{layout:"centered"},tags:["autodocs"]},a={args:{status:"success",children:"Active"}},r={args:{status:"warning",children:"Pending"}},n={args:{status:"error",children:"Failed"}},t={args:{status:"success",icon:"✓",children:"Completed"}},c={args:{status:"warning",icon:"⚠️",children:"Warning"}},i={args:{status:"error",icon:"✗",children:"Error"}},d={render:()=>e.jsxs("div",{style:{display:"flex",gap:"12px",flexWrap:"wrap"},children:[e.jsx(s,{status:"success",children:"Success"}),e.jsx(s,{status:"warning",children:"Warning"}),e.jsx(s,{status:"error",children:"Error"})]})},o={render:()=>e.jsxs("div",{style:{display:"flex",gap:"12px",flexWrap:"wrap"},children:[e.jsx(s,{status:"success",icon:"✓",children:"Active"}),e.jsx(s,{status:"warning",icon:"⚠️",children:"Pending"}),e.jsx(s,{status:"error",icon:"✗",children:"Failed"})]})},l={render:()=>e.jsxs("div",{style:{display:"flex",flexDirection:"column",gap:"16px"},children:[e.jsxs("div",{style:{display:"flex",alignItems:"center",gap:"8px"},children:[e.jsx("span",{children:"Service Status:"}),e.jsx(s,{status:"success",icon:"✓",children:"Online"})]}),e.jsxs("div",{style:{display:"flex",alignItems:"center",gap:"8px"},children:[e.jsx("span",{children:"Build Status:"}),e.jsx(s,{status:"warning",icon:"⏳",children:"In Progress"})]}),e.jsxs("div",{style:{display:"flex",alignItems:"center",gap:"8px"},children:[e.jsx("span",{children:"Test Results:"}),e.jsx(s,{status:"error",icon:"✗",children:"Failed"})]})]})},u={render:()=>e.jsxs("div",{style:{display:"flex",gap:"12px"},children:[e.jsx(s,{status:"success",children:"99%"}),e.jsx(s,{status:"warning",children:"12"}),e.jsx(s,{status:"error",children:"0"})]})};var g,m,x;a.parameters={...a.parameters,docs:{...(g=a.parameters)==null?void 0:g.docs,source:{originalSource:`{
  args: {
    status: 'success',
    children: 'Active'
  }
}`,...(x=(m=a.parameters)==null?void 0:m.docs)==null?void 0:x.source}}};var h,B,y;r.parameters={...r.parameters,docs:{...(h=r.parameters)==null?void 0:h.docs,source:{originalSource:`{
  args: {
    status: 'warning',
    children: 'Pending'
  }
}`,...(y=(B=r.parameters)==null?void 0:B.docs)==null?void 0:y.source}}};var v,f,j;n.parameters={...n.parameters,docs:{...(v=n.parameters)==null?void 0:v.docs,source:{originalSource:`{
  args: {
    status: 'error',
    children: 'Failed'
  }
}`,...(j=(f=n.parameters)==null?void 0:f.docs)==null?void 0:j.source}}};var S,w,W;t.parameters={...t.parameters,docs:{...(S=t.parameters)==null?void 0:S.docs,source:{originalSource:`{
  args: {
    status: 'success',
    icon: '✓',
    children: 'Completed'
  }
}`,...(W=(w=t.parameters)==null?void 0:w.docs)==null?void 0:W.source}}};var I,E,N;c.parameters={...c.parameters,docs:{...(I=c.parameters)==null?void 0:I.docs,source:{originalSource:`{
  args: {
    status: 'warning',
    icon: '⚠️',
    children: 'Warning'
  }
}`,...(N=(E=c.parameters)==null?void 0:E.docs)==null?void 0:N.source}}};var _,A,F;i.parameters={...i.parameters,docs:{...(_=i.parameters)==null?void 0:_.docs,source:{originalSource:`{
  args: {
    status: 'error',
    icon: '✗',
    children: 'Error'
  }
}`,...(F=(A=i.parameters)==null?void 0:A.docs)==null?void 0:F.source}}};var b,P,T;d.parameters={...d.parameters,docs:{...(b=d.parameters)==null?void 0:b.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap'
  }}>
      <Badge status="success">Success</Badge>
      <Badge status="warning">Warning</Badge>
      <Badge status="error">Error</Badge>
    </div>
}`,...(T=(P=d.parameters)==null?void 0:P.docs)==null?void 0:T.source}}};var O,R,q;o.parameters={...o.parameters,docs:{...(O=o.parameters)==null?void 0:O.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap'
  }}>
      <Badge status="success" icon="✓">Active</Badge>
      <Badge status="warning" icon="⚠️">Pending</Badge>
      <Badge status="error" icon="✗">Failed</Badge>
    </div>
}`,...(q=(R=o.parameters)==null?void 0:R.docs)==null?void 0:q.source}}};var V,C,D;l.parameters={...l.parameters,docs:{...(V=l.parameters)==null?void 0:V.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    flexDirection: 'column',
    gap: '16px'
  }}>
      <div style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px'
    }}>
        <span>Service Status:</span>
        <Badge status="success" icon="✓">Online</Badge>
      </div>
      <div style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px'
    }}>
        <span>Build Status:</span>
        <Badge status="warning" icon="⏳">In Progress</Badge>
      </div>
      <div style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px'
    }}>
        <span>Test Results:</span>
        <Badge status="error" icon="✗">Failed</Badge>
      </div>
    </div>
}`,...(D=(C=l.parameters)==null?void 0:C.docs)==null?void 0:D.source}}};var $,k,z;u.parameters={...u.parameters,docs:{...($=u.parameters)==null?void 0:$.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    gap: '12px'
  }}>
      <Badge status="success">99%</Badge>
      <Badge status="warning">12</Badge>
      <Badge status="error">0</Badge>
    </div>
}`,...(z=(k=u.parameters)==null?void 0:k.docs)==null?void 0:z.source}}};const X=["Success","Warning","Error","WithIcon","WarningWithIcon","ErrorWithIcon","AllVariants","WithIcons","StatusIndicators","Numbers"];export{d as AllVariants,n as Error,i as ErrorWithIcon,u as Numbers,l as StatusIndicators,a as Success,r as Warning,c as WarningWithIcon,t as WithIcon,o as WithIcons,X as __namedExportsOrder,U as default};
