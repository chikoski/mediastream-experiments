const base64Compile = str => WebAssembly.compile(Uint8Array.from(atob(str), b => b.charCodeAt(0)));

function clampGuest(i, min, max) {
  if (i < min || i > max) throw new TypeError(`must be between ${min} and ${max}`);
  return i;
}

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const emptyFunc = () => {};

const fetchCompile = url => fetch(url).then(WebAssembly.compileStreaming);

function finalizationRegistryCreate (unregister) {
  if (typeof FinalizationRegistry === 'undefined') {
    return { unregister () {} };
  }
  return new FinalizationRegistry(unregister);
}

const handleTables = [];

const instantiateCore = WebAssembly.instantiate;

const T_FLAG = 1 << 30;

function rscTableCreateOwn (table, rep) {
  const free = table[0] & ~T_FLAG;
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove (table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0) throw new TypeError('Invalid handle');
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}

const symbolRscHandle = Symbol('handle');

const symbolDispose = Symbol.dispose || Symbol.for('dispose');

function toUint32(val) {
  return val >>> 0;
}

function toUint8(val) {
  val >>>= 0;
  val %= 2 ** 8;
  return val;
}


let exports0;
let exports1;
let exports2;
let memory0;
let postReturn0;
let realloc0;
let postReturn1;
const handleTable0 = [T_FLAG, 0];
const finalizationRegistry0 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable0, handle);
  exports0['0'](rep);
});

handleTables[0] = handleTable0;

class ScanLine{
  constructor () {
    throw new Error('"ScanLine" resource does not define a constructor');
  }
}

ScanLine.prototype.getFilterType = function getFilterType() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.get-filter-type'](handle0);
  let enum2;
  switch (ret) {
    case 0: {
      enum2 = 'none';
      break;
    }
    case 1: {
      enum2 = 'sub';
      break;
    }
    case 2: {
      enum2 = 'up';
      break;
    }
    case 3: {
      enum2 = 'average';
      break;
    }
    case 4: {
      enum2 = 'paeth';
      break;
    }
    default: {
      throw new TypeError('invalid discriminant specified for FilterType');
    }
  }
  return enum2;
};

ScanLine.prototype.setFilterType = function setFilterType(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  var val2 = arg1;
  let enum2;
  switch (val2) {
    case 'none': {
      enum2 = 0;
      break;
    }
    case 'sub': {
      enum2 = 1;
      break;
    }
    case 'up': {
      enum2 = 2;
      break;
    }
    case 'average': {
      enum2 = 3;
      break;
    }
    case 'paeth': {
      enum2 = 4;
      break;
    }
    default: {
      if ((arg1) instanceof Error) {
        console.error(arg1);
      }
      
      throw new TypeError(`"${val2}" is not one of the cases of filter-type`);
    }
  }
  exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.set-filter-type'](handle0, enum2);
};

ScanLine.prototype.size = function size() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.size'](handle0);
  return ret >>> 0;
};

ScanLine.prototype.getPixelAt = function getPixelAt(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.get-pixel-at'](handle0, toUint32(arg1));
  return clampGuest(ret, 0, 255);
};

ScanLine.prototype.setPixelAt = function setPixelAt(arg1, arg2) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.set-pixel-at'](handle0, toUint32(arg1), toUint8(arg2));
};

ScanLine.prototype.read = function read() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.read'](handle0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var ptr2 = dataView(memory0).getInt32(ret + 4, true);
      var len2 = dataView(memory0).getInt32(ret + 8, true);
      var result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
      variant3= {
        tag: 'ok',
        val: result2
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  const retVal = variant3;
  postReturn0(ret);
  if (typeof retVal === 'object' && retVal.tag === 'err') {
    throw new ComponentError(retVal.val);
  }
  return retVal.val;
};

ScanLine.prototype.write = function write(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable0[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  var val2 = arg1;
  var len2 = val2.byteLength;
  var ptr2 = realloc0(0, 0, 1, len2 * 1);
  var src2 = new Uint8Array(val2.buffer || val2, val2.byteOffset, len2 * 1);
  (new Uint8Array(memory0.buffer, ptr2, len2 * 1)).set(src2);
  exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]scan-line.write'](handle0, ptr2, len2);
};
const handleTable1 = [T_FLAG, 0];
const finalizationRegistry1 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable1, handle);
  exports0['1'](rep);
});

handleTables[1] = handleTable1;

class Png{
  constructor () {
    throw new Error('"Png" resource does not define a constructor');
  }
}

Png.prototype.getScanLines = function getScanLines() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable1[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "Png" resource.');
  }
  var handle0 = handleTable1[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]png.get-scan-lines'](handle0);
  var len4 = dataView(memory0).getInt32(ret + 4, true);
  var base4 = dataView(memory0).getInt32(ret + 0, true);
  var result4 = [];
  for (let i = 0; i < len4; i++) {
    const base = base4 + i * 4;
    var handle3 = dataView(memory0).getInt32(base + 0, true);
    var rsc2 = new.target === ScanLine ? this : Object.create(ScanLine.prototype);
    Object.defineProperty(rsc2, symbolRscHandle, { writable: true, value: handle3});
    finalizationRegistry0.register(rsc2, handle3, rsc2);
    Object.defineProperty(rsc2, symbolDispose, { writable: true, value: function () {
      finalizationRegistry0.unregister(rsc2);
      rscTableRemove(handleTable0, handle3);
      rsc2[symbolDispose] = emptyFunc;
      rsc2[symbolRscHandle] = undefined;
      exports0['0'](handleTable0[(handle3 << 1) + 1] & ~T_FLAG);
    }});
    result4.push(rsc2);
  }
  const retVal = result4;
  postReturn1(ret);
  return retVal;
};

Png.prototype.read = function read() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable1[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "Png" resource.');
  }
  var handle0 = handleTable1[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[method]png.read'](handle0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var ptr2 = dataView(memory0).getInt32(ret + 4, true);
      var len2 = dataView(memory0).getInt32(ret + 8, true);
      var result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
      variant3= {
        tag: 'ok',
        val: result2
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  const retVal = variant3;
  postReturn0(ret);
  if (typeof retVal === 'object' && retVal.tag === 'err') {
    throw new ComponentError(retVal.val);
  }
  return retVal.val;
};

Png.create = function create(arg0, arg1, arg2) {
  var val0 = arg0;
  var len0 = val0.byteLength;
  var ptr0 = realloc0(0, 0, 1, len0 * 1);
  var src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
  const ret = exports1['chikoski:glitch-art/png-glitchable@0.3.5#[static]png.create'](ptr0, len0, toUint32(arg1), toUint32(arg2));
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var handle2 = dataView(memory0).getInt32(ret + 4, true);
      var rsc1 = new.target === Png ? this : Object.create(Png.prototype);
      Object.defineProperty(rsc1, symbolRscHandle, { writable: true, value: handle2});
      finalizationRegistry1.register(rsc1, handle2, rsc1);
      Object.defineProperty(rsc1, symbolDispose, { writable: true, value: function () {
        finalizationRegistry1.unregister(rsc1);
        rscTableRemove(handleTable1, handle2);
        rsc1[symbolDispose] = emptyFunc;
        rsc1[symbolRscHandle] = undefined;
        exports0['1'](handleTable1[(handle2 << 1) + 1] & ~T_FLAG);
      }});
      variant3= {
        tag: 'ok',
        val: rsc1
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  const retVal = variant3;
  if (typeof retVal === 'object' && retVal.tag === 'err') {
    throw new ComponentError(retVal.val);
  }
  return retVal.val;
};
const trampoline0 = rscTableCreateOwn.bind(null, handleTable0);
function trampoline1(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    exports0['0'](handleEntry.rep);
  }
}
const trampoline2 = rscTableCreateOwn.bind(null, handleTable1);
function trampoline3(handle) {
  const handleEntry = rscTableRemove(handleTable1, handle);
  if (handleEntry.own) {
    
    exports0['1'](handleEntry.rep);
  }
}

const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./png-glitch.core.wasm', import.meta.url));
    const module1 = base64Compile('AGFzbQEAAAABBQFgAX8AAwMCAAAEBQFwAQICBxQDATAAAAExAAEIJGltcG9ydHMBAAoVAgkAIABBABEAAAsJACAAQQERAAALAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yMTYuMACZAQRuYW1lABMSd2l0LWNvbXBvbmVudDpzaGltAX0CAD9kdG9yLVtleHBvcnRdY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNS1zY2FuLWxpbmUBOWR0b3ItW2V4cG9ydF1jaGlrb3NraTpnbGl0Y2gtYXJ0L3BuZy1nbGl0Y2hhYmxlQDAuMy41LXBuZw');
    const module2 = base64Compile('AGFzbQEAAAABBQFgAX8AAhoDAAEwAAAAATEAAAAIJGltcG9ydHMBcAECAgkIAQBBAAsCAAEALwlwcm9kdWNlcnMBDHByb2Nlc3NlZC1ieQENd2l0LWNvbXBvbmVudAcwLjIxNi4wABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
    ({ exports: exports0 } = yield instantiateCore(yield module1));
    ({ exports: exports1 } = yield instantiateCore(yield module0, {
      '[export]chikoski:glitch-art/png-glitchable@0.3.5': {
        '[resource-drop]png': trampoline3,
        '[resource-drop]scan-line': trampoline1,
        '[resource-new]png': trampoline2,
        '[resource-new]scan-line': trampoline0,
      },
    }));
    ({ exports: exports2 } = yield instantiateCore(yield module2, {
      '': {
        $imports: exports0.$imports,
        '0': exports1['chikoski:glitch-art/png-glitchable@0.3.5#[dtor]scan-line'],
        '1': exports1['chikoski:glitch-art/png-glitchable@0.3.5#[dtor]png'],
      },
    }));
    memory0 = exports1.memory;
    postReturn0 = exports1['cabi_post_chikoski:glitch-art/png-glitchable@0.3.5#[method]png.read'];
    realloc0 = exports1.cabi_realloc;
    postReturn1 = exports1['cabi_post_chikoski:glitch-art/png-glitchable@0.3.5#[method]png.get-scan-lines'];
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(runNext, reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

await $init;
const pngGlitchable035 = {
  Png: Png,
  ScanLine: ScanLine,
  
};

export { pngGlitchable035 as pngGlitchable, pngGlitchable035 as 'chikoski:glitch-art/png-glitchable@0.3.5',  }