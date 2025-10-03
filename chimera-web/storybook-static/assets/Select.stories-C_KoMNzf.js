import{j as l}from"./jsx-runtime-DF2Pcvd1.js";import{r as n}from"./index-B2-qRKKC.js";import"./_commonjsHelpers-Cpj98o6Y.js";function S({options:r,value:s,onChange:p,placeholder:G="Select...",disabled:u=!1,className:J=""}){const[a,o]=n.useState(!1),[i,c]=n.useState(-1),y=n.useRef(null),w=n.useRef(null),_=r.find(e=>e.value===s),d=r.filter(e=>!e.disabled);n.useEffect(()=>{const e=t=>{y.current&&!y.current.contains(t.target)&&o(!1)};if(a)return document.addEventListener("mousedown",e),()=>document.removeEventListener("mousedown",e)},[a]),n.useEffect(()=>{if(a){const e=d.findIndex(t=>t.value===s);c(e>=0?e:0)}},[a,s,d]);const Q=()=>{u||o(!a)},D=e=>{p==null||p(e),o(!1)},X=e=>{if(!u)switch(e.key){case"Enter":case" ":e.preventDefault(),a?i>=0&&D(d[i].value):o(!0);break;case"ArrowDown":e.preventDefault(),a?c(t=>t<d.length-1?t+1:t):o(!0);break;case"ArrowUp":e.preventDefault(),a&&c(t=>t>0?t-1:t);break;case"Escape":e.preventDefault(),o(!1);break;case"Home":a&&(e.preventDefault(),c(0));break;case"End":a&&(e.preventDefault(),c(d.length-1));break}};n.useEffect(()=>{if(a&&w.current&&i>=0){const e=w.current.children[i];e&&typeof e.scrollIntoView=="function"&&e.scrollIntoView({block:"nearest"})}},[i,a]);const Y=["chimera-select",a&&"chimera-select--open",u&&"chimera-select--disabled",J].filter(Boolean).join(" ");return l.jsxs("div",{ref:y,className:Y,onKeyDown:X,role:"combobox","aria-expanded":a,"aria-haspopup":"listbox","aria-disabled":u,tabIndex:u?-1:0,children:[l.jsxs("div",{className:"chimera-select__trigger",onClick:Q,children:[l.jsx("span",{className:"chimera-select__value",children:_?_.label:G}),l.jsx("span",{className:"chimera-select__arrow","aria-hidden":"true",children:a?"▲":"▼"})]}),a&&l.jsx("ul",{ref:w,className:"chimera-select__dropdown",role:"listbox","aria-label":"Options",children:r.map(e=>{const t=d.indexOf(e),Z=t===i,k=e.value===s;return l.jsx("li",{className:["chimera-select__option",Z&&"chimera-select__option--highlighted",k&&"chimera-select__option--selected",e.disabled&&"chimera-select__option--disabled"].filter(Boolean).join(" "),role:"option","aria-selected":k,"aria-disabled":e.disabled,onClick:()=>!e.disabled&&D(e.value),onMouseEnter:()=>!e.disabled&&c(t),children:e.label},e.value)})})]})}S.__docgenInfo={description:`Select dropdown component with keyboard navigation support.
Supports Arrow Up/Down, Enter, and Escape keys.
Fully accessible with ARIA attributes.

@example
\`\`\`tsx
<Select
  options={[
    { value: '1', label: 'Option 1' },
    { value: '2', label: 'Option 2' },
  ]}
  value={selected}
  onChange={setSelected}
  placeholder="Select an option"
/>
\`\`\``,methods:[],displayName:"Select",props:{options:{required:!0,tsType:{name:"Array",elements:[{name:"SelectOption"}],raw:"SelectOption[]"},description:"Array of options"},value:{required:!1,tsType:{name:"string"},description:"Currently selected value"},onChange:{required:!1,tsType:{name:"signature",type:"function",raw:"(value: string) => void",signature:{arguments:[{type:{name:"string"},name:"value"}],return:{name:"void"}}},description:"Callback when value changes"},placeholder:{required:!1,tsType:{name:"string"},description:"Placeholder text when no value selected",defaultValue:{value:"'Select...'",computed:!1}},disabled:{required:!1,tsType:{name:"boolean"},description:"Whether select is disabled",defaultValue:{value:"false",computed:!1}},className:{required:!1,tsType:{name:"string"},description:"Optional custom class name",defaultValue:{value:"''",computed:!1}}}};const le={title:"Components/Select",component:S,parameters:{layout:"centered"},tags:["autodocs"],argTypes:{disabled:{control:"boolean",description:"Disables the select"},placeholder:{control:"text",description:"Placeholder text"}}},O=[{value:"1",label:"Option 1"},{value:"2",label:"Option 2"},{value:"3",label:"Option 3"},{value:"4",label:"Option 4"},{value:"5",label:"Option 5"}],m={args:{options:O,placeholder:"Select an option..."}},h={args:{options:O,value:"2",placeholder:"Select an option..."}},b={args:{options:O,disabled:!0,placeholder:"Disabled select"}},f={args:{options:[{value:"1",label:"Option 1"},{value:"2",label:"Option 2 (disabled)",disabled:!0},{value:"3",label:"Option 3"},{value:"4",label:"Option 4 (disabled)",disabled:!0},{value:"5",label:"Option 5"}],placeholder:"Select an option..."}},v={args:{options:Array.from({length:20},(r,s)=>({value:`${s+1}`,label:`Option ${s+1}`})),placeholder:"Select from many options..."}},g={render:()=>{const[r,s]=n.useState(""),p=[{value:"react",label:"React"},{value:"vue",label:"Vue"},{value:"angular",label:"Angular"},{value:"svelte",label:"Svelte"}];return l.jsxs("div",{style:{width:"300px"},children:[l.jsx("div",{style:{marginBottom:"16px"},children:l.jsx(S,{options:p,value:r,onChange:s,placeholder:"Choose your framework"})}),l.jsxs("div",{style:{padding:"12px",background:"var(--bg-overlay)",borderRadius:"4px",fontSize:"14px",color:"var(--text-muted)"},children:["Selected: ",r||"(none)"]})]})}},x={render:()=>l.jsx("div",{style:{width:"400px"},children:l.jsx(S,{options:O,placeholder:"Full width select",className:"custom-select"})})};var j,E,I;m.parameters={...m.parameters,docs:{...(j=m.parameters)==null?void 0:j.docs,source:{originalSource:`{
  args: {
    options: defaultOptions,
    placeholder: 'Select an option...'
  }
}`,...(I=(E=m.parameters)==null?void 0:E.docs)==null?void 0:I.source}}};var A,C,N;h.parameters={...h.parameters,docs:{...(A=h.parameters)==null?void 0:A.docs,source:{originalSource:`{
  args: {
    options: defaultOptions,
    value: '2',
    placeholder: 'Select an option...'
  }
}`,...(N=(C=h.parameters)==null?void 0:C.docs)==null?void 0:N.source}}};var R,V,T;b.parameters={...b.parameters,docs:{...(R=b.parameters)==null?void 0:R.docs,source:{originalSource:`{
  args: {
    options: defaultOptions,
    disabled: true,
    placeholder: 'Disabled select'
  }
}`,...(T=(V=b.parameters)==null?void 0:V.docs)==null?void 0:T.source}}};var W,q,B;f.parameters={...f.parameters,docs:{...(W=f.parameters)==null?void 0:W.docs,source:{originalSource:`{
  args: {
    options: [{
      value: '1',
      label: 'Option 1'
    }, {
      value: '2',
      label: 'Option 2 (disabled)',
      disabled: true
    }, {
      value: '3',
      label: 'Option 3'
    }, {
      value: '4',
      label: 'Option 4 (disabled)',
      disabled: true
    }, {
      value: '5',
      label: 'Option 5'
    }],
    placeholder: 'Select an option...'
  }
}`,...(B=(q=f.parameters)==null?void 0:q.docs)==null?void 0:B.source}}};var $,F,H;v.parameters={...v.parameters,docs:{...($=v.parameters)==null?void 0:$.docs,source:{originalSource:`{
  args: {
    options: Array.from({
      length: 20
    }, (_, i) => ({
      value: \`\${i + 1}\`,
      label: \`Option \${i + 1}\`
    })),
    placeholder: 'Select from many options...'
  }
}`,...(H=(F=v.parameters)==null?void 0:F.docs)==null?void 0:H.source}}};var M,z,K;g.parameters={...g.parameters,docs:{...(M=g.parameters)==null?void 0:M.docs,source:{originalSource:`{
  render: () => {
    const [selected, setSelected] = useState('');
    const options: SelectOption[] = [{
      value: 'react',
      label: 'React'
    }, {
      value: 'vue',
      label: 'Vue'
    }, {
      value: 'angular',
      label: 'Angular'
    }, {
      value: 'svelte',
      label: 'Svelte'
    }];
    return <div style={{
      width: '300px'
    }}>
        <div style={{
        marginBottom: '16px'
      }}>
          <Select options={options} value={selected} onChange={setSelected} placeholder="Choose your framework" />
        </div>
        <div style={{
        padding: '12px',
        background: 'var(--bg-overlay)',
        borderRadius: '4px',
        fontSize: '14px',
        color: 'var(--text-muted)'
      }}>
          Selected: {selected || '(none)'}
        </div>
      </div>;
  }
}`,...(K=(z=g.parameters)==null?void 0:z.docs)==null?void 0:K.source}}};var L,P,U;x.parameters={...x.parameters,docs:{...(L=x.parameters)==null?void 0:L.docs,source:{originalSource:`{
  render: () => <div style={{
    width: '400px'
  }}>
      <Select options={defaultOptions} placeholder="Full width select" className="custom-select" />
    </div>
}`,...(U=(P=x.parameters)==null?void 0:P.docs)==null?void 0:U.source}}};const se=["Default","WithSelectedValue","Disabled","WithDisabledOptions","ManyOptions","Interactive","CustomWidth"];export{x as CustomWidth,m as Default,b as Disabled,g as Interactive,v as ManyOptions,f as WithDisabledOptions,h as WithSelectedValue,se as __namedExportsOrder,le as default};
