import{j as e}from"./jsx-runtime-DF2Pcvd1.js";import{r as M}from"./index-B2-qRKKC.js";import"./_commonjsHelpers-Cpj98o6Y.js";function a({title:u,children:W,footer:h,collapsible:t=!1,defaultCollapsed:F=!1,className:O=""}){const[n,V]=M.useState(F),m=()=>{t&&V(!n)},A=["chimera-panel",n&&"chimera-panel--collapsed",O].filter(Boolean).join(" ");return e.jsxs("div",{className:A,children:[u&&e.jsxs("div",{className:"chimera-panel__header",onClick:m,role:t?"button":void 0,tabIndex:t?0:void 0,onKeyDown:t?d=>{(d.key==="Enter"||d.key===" ")&&(d.preventDefault(),m())}:void 0,"aria-expanded":t?!n:void 0,children:[e.jsx("h3",{className:"chimera-panel__title",children:u}),t&&e.jsx("span",{className:"chimera-panel__toggle","aria-hidden":"true",children:n?"▶":"▼"})]}),!n&&e.jsxs(e.Fragment,{children:[e.jsx("div",{className:"chimera-panel__content",children:W}),h&&e.jsx("div",{className:"chimera-panel__footer",children:h})]})]})}a.__docgenInfo={description:`Panel container component with optional header and footer.
Supports collapsible functionality.

@example
\`\`\`tsx
<Panel title="Settings" footer={<Button>Save</Button>}>
  <p>Panel content here</p>
</Panel>
\`\`\``,methods:[],displayName:"Panel",props:{title:{required:!1,tsType:{name:"string"},description:"Panel title (header)"},children:{required:!0,tsType:{name:"ReactNode"},description:"Panel content"},footer:{required:!1,tsType:{name:"ReactNode"},description:"Optional footer content"},collapsible:{required:!1,tsType:{name:"boolean"},description:`Whether panel is collapsible
@default false`,defaultValue:{value:"false",computed:!1}},defaultCollapsed:{required:!1,tsType:{name:"boolean"},description:`Initial collapsed state (only used if collapsible is true)
@default false`,defaultValue:{value:"false",computed:!1}},className:{required:!1,tsType:{name:"string"},description:"Optional custom class name",defaultValue:{value:"''",computed:!1}}}};const G={title:"Components/Panel",component:a,parameters:{layout:"padded"},tags:["autodocs"]},l={args:{children:e.jsx("p",{children:"This is panel content. It can contain any React elements."})}},s={args:{title:"Panel Title",children:e.jsx("p",{children:"Panel content with a title header."})}},i={args:{title:"Settings Panel",children:e.jsxs("div",{children:[e.jsx("p",{children:"Configure your settings here."}),e.jsxs("label",{style:{display:"block",marginTop:"10px"},children:[e.jsx("input",{type:"checkbox"})," Enable notifications"]})]}),footer:e.jsxs("div",{style:{display:"flex",gap:"8px"},children:[e.jsx("button",{style:{padding:"6px 12px"},children:"Cancel"}),e.jsx("button",{style:{padding:"6px 12px",background:"var(--accent)"},children:"Save"})]})}},r={args:{title:"Collapsible Panel",collapsible:!0,children:e.jsx("p",{children:"Click the header to collapse/expand this panel."})}},o={args:{title:"Initially Collapsed",collapsible:!0,defaultCollapsed:!0,children:e.jsx("p",{children:"This panel starts collapsed."})}},c={args:{title:"Statistics",children:e.jsxs("div",{children:[e.jsxs("div",{style:{marginBottom:"12px"},children:[e.jsx("strong",{children:"Total Users:"})," 1,234"]}),e.jsxs("div",{style:{marginBottom:"12px"},children:[e.jsx("strong",{children:"Active Sessions:"})," 89"]}),e.jsxs("div",{children:[e.jsx("strong",{children:"Success Rate:"})," 99.5%"]})]})}},p={render:()=>e.jsxs("div",{style:{display:"flex",flexDirection:"column",gap:"16px",width:"400px"},children:[e.jsx(a,{title:"Section 1",collapsible:!0,children:e.jsx("p",{children:"Content for section 1"})}),e.jsx(a,{title:"Section 2",collapsible:!0,children:e.jsx("p",{children:"Content for section 2"})}),e.jsx(a,{title:"Section 3",collapsible:!0,defaultCollapsed:!0,children:e.jsx("p",{children:"Content for section 3 (initially collapsed)"})})]})};var x,f,g;l.parameters={...l.parameters,docs:{...(x=l.parameters)==null?void 0:x.docs,source:{originalSource:`{
  args: {
    children: <p>This is panel content. It can contain any React elements.</p>
  }
}`,...(g=(f=l.parameters)==null?void 0:f.docs)==null?void 0:g.source}}};var y,v,j;s.parameters={...s.parameters,docs:{...(y=s.parameters)==null?void 0:y.docs,source:{originalSource:`{
  args: {
    title: 'Panel Title',
    children: <p>Panel content with a title header.</p>
  }
}`,...(j=(v=s.parameters)==null?void 0:v.docs)==null?void 0:j.source}}};var b,C,S;i.parameters={...i.parameters,docs:{...(b=i.parameters)==null?void 0:b.docs,source:{originalSource:`{
  args: {
    title: 'Settings Panel',
    children: <div>
        <p>Configure your settings here.</p>
        <label style={{
        display: 'block',
        marginTop: '10px'
      }}>
          <input type="checkbox" /> Enable notifications
        </label>
      </div>,
    footer: <div style={{
      display: 'flex',
      gap: '8px'
    }}>
        <button style={{
        padding: '6px 12px'
      }}>Cancel</button>
        <button style={{
        padding: '6px 12px',
        background: 'var(--accent)'
      }}>Save</button>
      </div>
  }
}`,...(S=(C=i.parameters)==null?void 0:C.docs)==null?void 0:S.source}}};var P,T,_;r.parameters={...r.parameters,docs:{...(P=r.parameters)==null?void 0:P.docs,source:{originalSource:`{
  args: {
    title: 'Collapsible Panel',
    collapsible: true,
    children: <p>Click the header to collapse/expand this panel.</p>
  }
}`,...(_=(T=r.parameters)==null?void 0:T.docs)==null?void 0:_.source}}};var k,N,B;o.parameters={...o.parameters,docs:{...(k=o.parameters)==null?void 0:k.docs,source:{originalSource:`{
  args: {
    title: 'Initially Collapsed',
    collapsible: true,
    defaultCollapsed: true,
    children: <p>This panel starts collapsed.</p>
  }
}`,...(B=(N=o.parameters)==null?void 0:N.docs)==null?void 0:B.source}}};var D,I,R;c.parameters={...c.parameters,docs:{...(D=c.parameters)==null?void 0:D.docs,source:{originalSource:`{
  args: {
    title: 'Statistics',
    children: <div>
        <div style={{
        marginBottom: '12px'
      }}>
          <strong>Total Users:</strong> 1,234
        </div>
        <div style={{
        marginBottom: '12px'
      }}>
          <strong>Active Sessions:</strong> 89
        </div>
        <div>
          <strong>Success Rate:</strong> 99.5%
        </div>
      </div>
  }
}`,...(R=(I=c.parameters)==null?void 0:I.docs)==null?void 0:R.source}}};var q,w,E;p.parameters={...p.parameters,docs:{...(q=p.parameters)==null?void 0:q.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    width: '400px'
  }}>
      <Panel title="Section 1" collapsible>
        <p>Content for section 1</p>
      </Panel>
      <Panel title="Section 2" collapsible>
        <p>Content for section 2</p>
      </Panel>
      <Panel title="Section 3" collapsible defaultCollapsed>
        <p>Content for section 3 (initially collapsed)</p>
      </Panel>
    </div>
}`,...(E=(w=p.parameters)==null?void 0:w.docs)==null?void 0:E.source}}};const H=["Default","WithTitle","WithFooter","Collapsible","CollapsedByDefault","ComplexContent","MultiplePanels"];export{o as CollapsedByDefault,r as Collapsible,c as ComplexContent,l as Default,p as MultiplePanels,i as WithFooter,s as WithTitle,H as __namedExportsOrder,G as default};
