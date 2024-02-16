import { reactive } from "vue";
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { getFromLocalStorage } from "~/utils";
import { emitter } from "~/main";

type InjectedExtension = Awaited<ReturnType<typeof web3Enable>>[number];
type InjectedAccountWithMeta = Awaited<ReturnType<typeof web3Accounts>>[number];

class AccountStore {
  api: ApiPromise | null = null;
  extensions: InjectedExtension[] = [];
  accounts: InjectedAccountWithMeta[] = [];
  activeAccount: string | null = null;
  loading = false;

  async init() {
    const wasConnected = !!getFromLocalStorage("account", "");
    if (wasConnected)
      this.connectToSigner();

    const apiResponse = await ApiPromise.create({
      provider: new WsProvider("wss://ws.test.azero.dev"),
    });
    this.api = apiResponse;
  }

  async injectSigner() {
    const injectedExtensions = await web3Enable("Llama Bet");
    this.extensions = injectedExtensions;
  }

  async connectToSigner() {
    this.loading = true;
    try {
      await this.injectSigner();
      const accounts = await web3Accounts(
        {
          extensions: ["aleph-zero-signer"],
          genesisHash: "0x05d5279c52c484cc80396535a316add7d47b1c5b9e0398dd1f584149341460c5",
        },
      );

      if (accounts.length > 0) {
        this.accounts = accounts;
        this.activeAccount = accounts[0].address;
        if (accounts.length > 1 && !getFromLocalStorage("account", ""))
          // There was no account chosen before, let user choose one
          emitter.emit("select-account");

        console.log(this.accounts);
        localStorage.setItem("account", this.activeAccount); // In case of refreshing the page, the user will be automatically reconnected
      }
    } catch (e) {
      console.error(e);
    }
    this.loading = false;
  }

  changeActiveAccount(address: string) {
    this.activeAccount = address;
    localStorage.setItem("account", this.activeAccount);
  }

  disconnect() {
    this.extensions = [];
    this.accounts = [];
    this.activeAccount = null;
    localStorage.removeItem("account");
  }
}

export const accountStore = reactive(new AccountStore()) as any as AccountStore;
