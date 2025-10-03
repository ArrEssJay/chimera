import{j as t}from"./jsx-runtime-DF2Pcvd1.js";import{r}from"./index-B2-qRKKC.js";import"./_commonjsHelpers-Cpj98o6Y.js";function e({content:q,placement:A="top",children:V,className:F=""}){const[m,o]=r.useState(!1),[u]=r.useState(()=>`tooltip-${Math.random().toString(36).substr(2,9)}`),n=r.useRef(),k=()=>{clearTimeout(n.current),n.current=window.setTimeout(()=>{o(!0)},200)},O=()=>{clearTimeout(n.current),o(!1)},P=()=>{o(!0)},W=()=>{o(!1)};r.useEffect(()=>()=>{clearTimeout(n.current)},[]);const $=["chimera-tooltip",F].filter(Boolean).join(" "),z=["chimera-tooltip__content",`chimera-tooltip__content--${A}`,m&&"chimera-tooltip__content--visible"].filter(Boolean).join(" ");return t.jsxs("div",{className:$,onMouseEnter:k,onMouseLeave:O,onFocus:P,onBlur:W,children:[t.jsx("div",{"aria-describedby":m?u:void 0,children:V}),t.jsx("div",{id:u,role:"tooltip",className:z,"aria-hidden":!m,children:q})]})}e.__docgenInfo={description:`Tooltip component with 4 placement options.
Shows on hover and focus.
Fully accessible with ARIA attributes.

@example
\`\`\`tsx
<Tooltip content="Helpful information" placement="top">
  <button>Hover me</button>
</Tooltip>
\`\`\``,methods:[],displayName:"Tooltip",props:{content:{required:!0,tsType:{name:"ReactNode"},description:"Tooltip content"},placement:{required:!1,tsType:{name:"union",raw:"'top' | 'right' | 'bottom' | 'left'",elements:[{name:"literal",value:"'top'"},{name:"literal",value:"'right'"},{name:"literal",value:"'bottom'"},{name:"literal",value:"'left'"}]},description:`Tooltip placement
@default 'top'`,defaultValue:{value:"'top'",computed:!1}},children:{required:!0,tsType:{name:"ReactNode"},description:"Element that triggers the tooltip"},className:{required:!1,tsType:{name:"string"},description:"Optional custom class name",defaultValue:{value:"''",computed:!1}}}};const K={title:"Components/Tooltip",component:e,parameters:{layout:"centered"},tags:["autodocs"]},i={args:{content:"This is a tooltip on top",placement:"top",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Hover me (top)"})}},s={args:{content:"This is a tooltip on bottom",placement:"bottom",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Hover me (bottom)"})}},a={args:{content:"This is a tooltip on left",placement:"left",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Hover me (left)"})}},p={args:{content:"This is a tooltip on right",placement:"right",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Hover me (right)"})}},l={args:{content:"This is a longer tooltip with more detailed information",placement:"top",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Long tooltip"})}},c={render:()=>t.jsxs("div",{style:{display:"grid",gridTemplateColumns:"repeat(2, 1fr)",gap:"60px",padding:"60px"},children:[t.jsx(e,{content:"Top tooltip",placement:"top",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Top"})}),t.jsx(e,{content:"Right tooltip",placement:"right",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Right"})}),t.jsx(e,{content:"Bottom tooltip",placement:"bottom",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Bottom"})}),t.jsx(e,{content:"Left tooltip",placement:"left",children:t.jsx("button",{style:{padding:"8px 16px"},children:"Left"})})]})},d={args:{content:"Click for more information",placement:"top",children:t.jsx("button",{style:{padding:"8px",borderRadius:"50%",width:"32px",height:"32px",display:"flex",alignItems:"center",justifyContent:"center"},children:"?"})}};var h,g,x;i.parameters={...i.parameters,docs:{...(h=i.parameters)==null?void 0:h.docs,source:{originalSource:`{
  args: {
    content: 'This is a tooltip on top',
    placement: 'top',
    children: <button style={{
      padding: '8px 16px'
    }}>Hover me (top)</button>
  }
}`,...(x=(g=i.parameters)==null?void 0:g.docs)==null?void 0:x.source}}};var b,f,T;s.parameters={...s.parameters,docs:{...(b=s.parameters)==null?void 0:b.docs,source:{originalSource:`{
  args: {
    content: 'This is a tooltip on bottom',
    placement: 'bottom',
    children: <button style={{
      padding: '8px 16px'
    }}>Hover me (bottom)</button>
  }
}`,...(T=(f=s.parameters)==null?void 0:f.docs)==null?void 0:T.source}}};var y,v,j;a.parameters={...a.parameters,docs:{...(y=a.parameters)==null?void 0:y.docs,source:{originalSource:`{
  args: {
    content: 'This is a tooltip on left',
    placement: 'left',
    children: <button style={{
      padding: '8px 16px'
    }}>Hover me (left)</button>
  }
}`,...(j=(v=a.parameters)==null?void 0:v.docs)==null?void 0:j.source}}};var R,L,S;p.parameters={...p.parameters,docs:{...(R=p.parameters)==null?void 0:R.docs,source:{originalSource:`{
  args: {
    content: 'This is a tooltip on right',
    placement: 'right',
    children: <button style={{
      padding: '8px 16px'
    }}>Hover me (right)</button>
  }
}`,...(S=(L=p.parameters)==null?void 0:L.docs)==null?void 0:S.source}}};var w,B,C;l.parameters={...l.parameters,docs:{...(w=l.parameters)==null?void 0:w.docs,source:{originalSource:`{
  args: {
    content: 'This is a longer tooltip with more detailed information',
    placement: 'top',
    children: <button style={{
      padding: '8px 16px'
    }}>Long tooltip</button>
  }
}`,...(C=(B=l.parameters)==null?void 0:B.docs)==null?void 0:C.source}}};var H,_,I;c.parameters={...c.parameters,docs:{...(H=c.parameters)==null?void 0:H.docs,source:{originalSource:`{
  render: () => <div style={{
    display: 'grid',
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '60px',
    padding: '60px'
  }}>
      <Tooltip content="Top tooltip" placement="top">
        <button style={{
        padding: '8px 16px'
      }}>Top</button>
      </Tooltip>
      <Tooltip content="Right tooltip" placement="right">
        <button style={{
        padding: '8px 16px'
      }}>Right</button>
      </Tooltip>
      <Tooltip content="Bottom tooltip" placement="bottom">
        <button style={{
        padding: '8px 16px'
      }}>Bottom</button>
      </Tooltip>
      <Tooltip content="Left tooltip" placement="left">
        <button style={{
        padding: '8px 16px'
      }}>Left</button>
      </Tooltip>
    </div>
}`,...(I=(_=c.parameters)==null?void 0:_.docs)==null?void 0:I.source}}};var E,N,M;d.parameters={...d.parameters,docs:{...(E=d.parameters)==null?void 0:E.docs,source:{originalSource:`{
  args: {
    content: 'Click for more information',
    placement: 'top',
    children: <button style={{
      padding: '8px',
      borderRadius: '50%',
      width: '32px',
      height: '32px',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center'
    }}>
        ?
      </button>
  }
}`,...(M=(N=d.parameters)==null?void 0:N.docs)==null?void 0:M.source}}};const Q=["Top","Bottom","Left","Right","LongContent","AllPlacements","WithIcon"];export{c as AllPlacements,s as Bottom,a as Left,l as LongContent,p as Right,i as Top,d as WithIcon,Q as __namedExportsOrder,K as default};
